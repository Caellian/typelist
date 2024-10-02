mod alphabet;
#[path = "./impl.rs"]
mod implementation;
mod mapping;
mod types;
mod util;
mod value;
mod variant;
mod constraints;

use implementation::*;
use proc_macro2::Span;
use syn::*;
use types::TupleT;
pub use util::ModuleVisibility;
use util::{crate_visibility, use_module, default};

use crate::{mapping::MAPPING_MODULE, util::AddModule, variant::VARIANT_MODULE};

pub struct Options {
    pub module: Option<(ModuleVisibility, String)>,
    pub size: usize,
    pub list_name: String,
}
impl Default for Options {
    fn default() -> Self {
        Options {
            module: None,
            size: 12,
            list_name: "TypeList".to_string(),
        }
    }
}

fn produce_items(options: Options) -> Vec<Item> {
    let mut items = Vec::new();
    if options.size == 0 {
        return items;
    }

    items.add_module(VARIANT_MODULE, &options);
    items.add_module(MAPPING_MODULE, &options);

    for size in 0..=options.size {
        let variant = TupleT::new(size);
        items.push(BaseImpl { target: &variant }.into_item());
        if size > 0 {
            items.push(NonEmptyImpl { target: &variant }.into_item());
        }
        for index in 0..size {
            items.push(
                HasElementImpl {
                    target: &variant,
                    index,
                }
                .into_item(),
            )
        }
    }

    if let Some((visibility, name)) = options.module {
        let mut result = Vec::with_capacity(2);
        result.push(Item::Mod(ItemMod {
            attrs: vec![],
            vis: match visibility {
                ModuleVisibility::Private | ModuleVisibility::Flat => Visibility::Inherited,
                ModuleVisibility::Public => Visibility::Public(default()),
                ModuleVisibility::Crate => crate_visibility(),
            },
            unsafety: None,
            mod_token: default(),
            ident: Ident::new(&name, Span::call_site()),
            content: Some((default(), items)),
            semi: default(),
        }));
        if visibility == ModuleVisibility::Flat {
            result.push(use_module(ModuleVisibility::Public, name))
        }
        result
    } else {
        items
    }
}

fn produce_file(options: Options) -> syn::File {
    syn::File {
        shebang: None,
        attrs: vec![],
        items: produce_items(options),
    }
}

pub fn generate(options: Options, target: impl AsRef<std::path::Path>) {
    let syntax_tree = produce_file(options);
    let formatted = prettyplease::unparse(&syntax_tree);
    std::fs::write(target.as_ref(), formatted).unwrap();
}
