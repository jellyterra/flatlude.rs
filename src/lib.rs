// Authored 2025 Jelly Terra <jellyterra@proton.me>

use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::PathBuf;

#[proc_macro]
pub fn flatlude(_input: TokenStream) -> TokenStream {
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(path) => path,
        Err(_) => {
            return TokenStream::from(quote! { compile_error!("Unable to read CARGO_MANIFEST_DIR environment variable."); });
        }
    };

    let src_dir = PathBuf::from(manifest_dir).join("src");

    let mut module_declarations = Vec::new();

    let entries = match fs::read_dir(&src_dir) {
        Ok(entries) => entries,
        Err(e) => {
            let error_msg = format!("Failed to read src directory [{}]: {}", src_dir.display(), e);
            return TokenStream::from(quote! { compile_error!(#error_msg); });
        }
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "rs" {
                    if let Some(stem) = path.file_stem() {
                        let mod_name = stem.to_string_lossy();
                        if mod_name != "lib" && mod_name != "main" {
                            let mod_name = syn::Ident::new(&mod_name, proc_macro2::Span::call_site());
                            module_declarations.push(quote! {
                                pub mod #mod_name;
                                pub use #mod_name::*;
                            });
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        #(#module_declarations)*
    };

    TokenStream::from(expanded)
}
