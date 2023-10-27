use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    token::{Colon, Semi, Static},
    Attribute, Ident, ItemFn, ItemStatic, LitStr, Result, StaticMutability, Token, Type,
    Visibility,
};
use syn::{parse_macro_input, ExprPath, LitInt};

struct ConstArray {
    size: usize,
    func: ExprPath,
}

impl Parse for ConstArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let func = input.parse()?;
        input.parse::<Token![;]>()?;
        let size = input.parse::<LitInt>()?.base10_parse()?;
        Ok(Self { size, func })
    }
}

#[proc_macro]
pub fn const_array_from_fn(input: TokenStream) -> TokenStream {
    let ConstArray { size, func } = parse_macro_input!(input as ConstArray);
    let range = 0usize..size;
    quote! { [
        #( #func(#range) ),*
    ] }
    .into()
}

struct EmptyAttr;

impl Parse for EmptyAttr {
    fn parse(_: ParseStream) -> Result<Self> {
        Ok(Self)
    }
}

struct ItemStaticImport {
    attrs: Vec<Attribute>,
    vis: Visibility,
    static_token: Static,
    mutability: StaticMutability,
    ident: Ident,
    colon_token: Colon,
    ty: Box<Type>,
    semi_token: Semi,
}

impl Parse for ItemStaticImport {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn rtos_import(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as EmptyAttr);
    let ItemStaticImport {
        attrs,
        vis,
        static_token,
        mutability,
        ident,
        colon_token,
        ty,
        semi_token,
    } = parse_macro_input!(item as ItemStaticImport);
    let link_name = format!("rtos.{ident}");
    quote! {
        extern "C" {
            #( #attrs )*
            #[link_name = #link_name]
            #vis #static_token #mutability #ident #colon_token #ty #semi_token
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn rtos_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as EmptyAttr);
    let item_static = parse_macro_input!(item as ItemStatic);
    let export_name = format!("rtos.{}", item_static.ident);
    quote! {
        #[export_name = #export_name]
        #item_static
    }
    .into()
}

#[proc_macro]
pub fn rtos_feature(input: TokenStream) -> TokenStream {
    let feature_name = parse_macro_input!(input as LitStr);
    let export_name = format!("rtos.feature.{}", feature_name.value());
    let ident_name = format_ident!(
        "RTOS_FEATURE_{}_MARKER",
        feature_name.value().to_uppercase()
    );
    quote! {
        #[doc(hidden)]
        #[cfg(feature = #feature_name)]
        #[export_name = #export_name]
        #[link_section = ".note.rtos.feature_flag"]
        #[used]
        static #ident_name: () = ();
    }
    .into()
}

#[proc_macro_attribute]
pub fn rtos_task_entry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as EmptyAttr);
    let mut entry_fn = parse_macro_input!(item as ItemFn);
    let fn_ident = entry_fn.sig.ident.clone();
    let inner_fn_ident = format_ident!("_inner_{}", fn_ident);
    entry_fn.sig.ident = inner_fn_ident.clone();

    quote! {
        #[naked]
        #[export_name = "_start"]
        unsafe extern "C" fn #fn_ident() -> ! {
            #entry_fn

            extern "C" {
                static _data_start: ::core::ffi::c_void;
                static _data_end: ::core::ffi::c_void;
                static _data_load: ::core::ffi::c_void;

                static _bss_start: ::core::ffi::c_void;
                static _bss_end: ::core::ffi::c_void;

                static _stack_end: ::core::ffi::c_void;
            }

            ::core::arch::asm!(
                // Set the task stack pointer
                "la sp, {_stack_end}",

                // Load .data from flash
                "la a0, {_data_start}",
                "la a1, {_data_end}",
                "la a2, {_data_load}",
                "bgeu a0, a1, 2f",
                "1:",
                "lw a3, (a2)",
                "sw a3, (a0)",
                "addi a0, a0, 4",
                "addi a2, a2, 4",
                "bltu a0, a1, 1b",
                "2:",

                // Zero .bss
                "la a0, {_bss_start}",
                "la a1, {_bss_end}",
                "bgeu a0, a1, 4f",
                "3:",
                "sw zero, (a0)",
                "addi a0, a0, 4",
                "bltu a0, a1, 3b",
                "4:",

                // Let's go!
                "j {task_main}",

                _data_start   = sym _data_start,
                _data_end     = sym _data_end,
                _data_load    = sym _data_load,
                _bss_start    = sym _bss_start,
                _bss_end      = sym _bss_end,
                _stack_end    = sym _stack_end,
                task_main     = sym #inner_fn_ident,
                options(noreturn))
        }
    }
    .into()
}
