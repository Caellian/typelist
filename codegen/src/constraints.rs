use proc_macro2::Span;
use syn::{*, punctuated::Punctuated};

use crate::util::default;

pub fn has_element(index: usize) -> Path {
    Path {
        leading_colon: None,
        segments: Punctuated::from_iter([
            PathSegment {
                ident: Ident::new("HasElement", Span::call_site()),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: default(),
                    args: Punctuated::from_iter([
                        GenericArgument::Const(Expr::Lit(ExprLit {
                            attrs: vec![],
                            lit: Lit::Int(LitInt::new(index.to_string().as_str(), Span::call_site())),
                        }))
                    ]),
                    gt_token: default(),
                }),
            }
        ])
    }
}
