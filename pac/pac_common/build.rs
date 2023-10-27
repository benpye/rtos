use std::{env, fs, path::Path};

use proc_macro2::TokenStream;
use quote::ToTokens;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_src = Path::new(&out_dir).join("lib.rs");

    let ir = chiptool::ir::IR::new();
    let options = chiptool::generate::Options {
        common_module: chiptool::generate::CommonModule::Builtin,
    };
    let common_module = chiptool::generate::render(&ir, &options).unwrap();

    let mut root = TokenStream::new();

    let common_module: syn::File = syn::parse2(common_module).unwrap();
    for i in common_module.items {
        i.to_tokens(&mut root)
    }

    let root_syn_file: syn::File = syn::parse2(root.into()).unwrap();
    let root_pretty = prettyplease::unparse(&root_syn_file);

    fs::write(out_src, root_pretty).unwrap();
}
