use std::{env, fs, path::Path, str::FromStr};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn main() {
    let modules = [("systick", "systick_v4.yaml"), ("pfic", "pfic_v4.yaml")];

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let out_src = Path::new(&out_dir).join("lib.rs");
    let data_dir = Path::new(&manifest_dir).join("data");

    let mut root = TokenStream::new();

    let validate_options = chiptool::validate::Options {
        allow_register_overlap: false,
        allow_field_overlap: false,
        allow_enum_dup_value: false,
        allow_unused_enums: true,
        allow_unused_fieldsets: false,
    };

    let generate_options = chiptool::generate::Options {
        common_module: chiptool::generate::CommonModule::External(
            TokenStream::from_str("pac_common").unwrap(),
        ),
    };

    for (module_name, file_name) in modules {
        let file_path = data_dir.join(file_name);
        println!("cargo:rerun-if-changed={}", file_path.display());

        let fd = fs::File::open(file_path).unwrap();
        let mut ir: chiptool::ir::IR = serde_yaml::from_reader(fd).unwrap();

        for err in chiptool::validate::validate(&ir, validate_options.clone()) {
            println!("cargo:warning={}: {}", file_name, err);
        }

        chiptool::transform::Sanitize {}.run(&mut ir).unwrap();
        chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();
        let module = chiptool::generate::render(&ir, &generate_options).unwrap();

        let module: syn::File = syn::parse2(module).unwrap();
        let module = module.items;

        let module_name = Ident::new(module_name, Span::call_site());
        root.extend(quote!(
            pub mod #module_name {
                #(#module)*
            }
        ));
    }

    let root_syn_file: syn::File = syn::parse2(root.into()).unwrap();
    let root_pretty = prettyplease::unparse(&root_syn_file);

    fs::write(out_src, root_pretty).unwrap();
}
