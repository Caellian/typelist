use quote::format_ident;
use syn::{*, punctuated::Punctuated};
use crate::{types::{TupleT, Pure}, util::{Module, ModuleVisibility}, Options};

pub struct TupleValueEnum<'a> {
    pub target: &'a TupleT<Pure>,
}

impl<'a> TupleValueEnum<'a> {
    pub fn into_item(self) -> Item {
        if self.target.is_empty() {
            return Item::Enum(parse_quote! {
                pub enum Tuple0 {}
            });
        }

        let name = format_ident!("Tuple{}", self.target.len());
        let generics = self.target.generics(None, None);
        let variants = self.target.iter_types().enumerate().map(|(i, value_t)| {
            let ident = format_ident!("Variant{}", i);
            Variant {
                attrs: vec![],
                ident,
                fields: Fields::Unnamed(FieldsUnnamed {
                    paren_token: Default::default(),
                    unnamed: Punctuated::from_iter(Some(
                        Field {
                            attrs: vec![],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: value_t,
                        }
                    )),
                }),
                discriminant: None,
            }
        });

        Item::Enum(parse_quote! {
            #[repr(usize, C)]
            pub enum #name #generics {
                #(#variants,)*
            }
        })
    }
}

pub(crate) const VARIANT_MODULE: Module = Module {
    visibility: ModuleVisibility::Public,
    name: "variant",
    generate: generate_module,
};
fn generate_module(items: &mut Vec<Item>, options: &Options) {
    for size in 0..=options.size {
        let variant = TupleT::new(size);
        items.push(TupleValueEnum { target: &variant }.into_item())
    }
}