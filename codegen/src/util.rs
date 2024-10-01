use std::borrow::Cow;

use proc_macro2::Span;
use syn::{punctuated::Punctuated, *};

use crate::Options;

macro_rules! spread {
    ($m: ident, $($t: tt),+ $(,)?) => {
        $(
            $m!($t);
        )*
    };
}

pub trait AsIdentPath {
    fn to_path(self) -> Path;
    #[inline]
    fn to_expr(self) -> Expr
    where
        Self: Sized,
    {
        Expr::Path(ExprPath {
            attrs: vec![],
            qself: None,
            path: self.to_path(),
        })
    }
    #[inline]
    fn to_type(self) -> Type
    where
        Self: Sized,
    {
        Type::Path(TypePath {
            qself: None,
            path: self.to_path(),
        })
    }
}

macro_rules! impl_as_str_ty {
    ({$($lt: lifetime),+ => $t: ty}) => {
        impl<$($lt,)+> AsIdentPath for $t {
            fn to_path(self) -> Path {
                Path::from(Ident::new(self.as_ref(), Span::call_site()))
            }
        }
    };
    ({$t: ty}) => {
        impl AsIdentPath for $t {
            fn to_path(self) -> Path {
                Path::from(Ident::new(self.as_ref(), Span::call_site()))
            }
        }
    };
}
spread![
    impl_as_str_ty,
    {String},
    {'a => &'a String},
    {'a => &'a str},
    {'a => Cow<'a, str>},
];
macro_rules! impl_as_str_list {
    ({$($lt: lifetime),* $(,)? $(const $c: ident : $ct: ty),* => $t: ty}) => {
        impl <$($lt,)* $(const $c: $ct,)*> AsIdentPath for $t {
            fn to_path(self) -> Path {
                Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(self.into_iter().map(|it| {
                        PathSegment {
                            ident: Ident::new(it.as_ref(), Span::call_site()),
                            arguments: PathArguments::None,
                        }
                    })),
                }
            }
        }
    };
    ({$t: ty}) => {
        impl AsIdentPath for $t {
            fn to_path(self) -> Path {
                Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(self.into_iter().map(|it| {
                        PathSegment {
                            ident: Ident::new(it.as_ref(), Span::call_site()),
                            arguments: PathArguments::None,
                        }
                    })),
                }
            }
        }
    };
}
spread![
    impl_as_str_list,
    {const N: usize => [String; N]},
    {'a, const N: usize => [&'a String; N]},
    {'a, const N: usize => [&'a str; N]},
    {'a, const N: usize => [Cow<'a, str>; N]},
    {'a => &'a [String]},
    {'a, 'b =>  &'a [&'b String]},
    {'a, 'b =>  &'a [&'b str]},
    {'a, 'b =>  &'a [Cow<'b, str>]},
];

impl<P, I> AsIdentPath for (P, I)
where
    P: AsIdentPath,
    I: IntoIterator<Item = GenericArgument>,
{
    fn to_path(self) -> Path {
        let mut result = self.0.to_path();
        if let Some(segment) = result.segments.last_mut() {
            segment.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: Punctuated::from_iter(self.1),
                gt_token: Default::default(),
            });
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleVisibility {
    Private,
    Public,
    Crate,
    Flat,
}
pub(crate) struct Module {
    pub visibility: ModuleVisibility,
    pub name: &'static str,
    pub generate: fn(&mut Vec<Item>, &Options),
}
pub(crate) trait AddModule {
    fn add_module(&mut self, module: Module, options: &Options);
}
impl AddModule for Vec<Item> {
    fn add_module(&mut self, module: Module, options: &Options) {
        self.push({
            let mut items = Vec::new();
            (module.generate)(&mut items, options);
            Item::Mod(ItemMod {
                attrs: vec![],
                vis: match module.visibility {
                    ModuleVisibility::Private | ModuleVisibility::Flat => Visibility::Inherited,
                    ModuleVisibility::Public => Visibility::Public(Default::default()),
                    ModuleVisibility::Crate => crate_visibility(),
                },
                unsafety: None,
                mod_token: Default::default(),
                ident: Ident::new(module.name, Span::call_site()),
                content: Some((Default::default(), items)),
                semi: None,
            })
        });
        if module.visibility == ModuleVisibility::Crate {
            return;
        }
        self.push(use_module(
            match module.visibility {
                ModuleVisibility::Public => ModuleVisibility::Private,
                keep => keep,
            },
            module.name,
        ));
    }
}

pub trait CollectGenerics {
    fn collect_generics(self, where_clause: Option<WhereClause>) -> Generics;
}
impl<I> CollectGenerics for I
where
    I: IntoIterator<Item = Ident>,
{
    fn collect_generics(self, where_clause: Option<WhereClause>) -> Generics {
        Generics {
            lt_token: Default::default(),
            params: Punctuated::from_iter(self.into_iter().map(|ident| {
                GenericParam::Type(TypeParam {
                    attrs: vec![],
                    ident,
                    colon_token: None,
                    bounds: Default::default(),
                    eq_token: None,
                    default: None,
                })
            })),
            gt_token: Default::default(),
            where_clause,
        }
    }
}

#[inline]
pub fn crate_visibility() -> Visibility {
    Visibility::Restricted(VisRestricted {
        pub_token: Default::default(),
        paren_token: Default::default(),
        in_token: None,
        path: Box::new("crate".to_path()),
    })
}

pub fn use_module(visibility: ModuleVisibility, name: impl AsRef<str>) -> Item {
    Item::Use(ItemUse {
        attrs: vec![],
        vis: match visibility {
            ModuleVisibility::Private => Visibility::Inherited,
            ModuleVisibility::Public | ModuleVisibility::Flat => {
                Visibility::Public(Default::default())
            }
            ModuleVisibility::Crate => crate_visibility(),
        },
        use_token: Default::default(),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new(name.as_ref(), Span::call_site()),
            colon2_token: Default::default(),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Default::default(),
            })),
        }),
        semi_token: Default::default(),
    })
}

#[inline]
pub fn ident_ty(ident: Ident) -> Type {
    Type::Path(TypePath { qself: None, path: Path::from(ident) })
}

#[inline]
pub fn index_tuple(tuple: &Expr, index: usize) -> Expr {
    Expr::Field(ExprField {
        attrs: vec![],
        base: Box::new(tuple.clone()),
        dot_token: Default::default(),
        member: Member::Unnamed(index.into()),
    })
}

fn function_bound(kind: &str, inputs: impl IntoIterator<Item = Type>, output: Type) -> TraitBound {
    TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter([PathSegment {
                ident: Ident::new(kind, Span::call_site()),
                arguments: PathArguments::Parenthesized(
                    ParenthesizedGenericArguments {
                        paren_token: Default::default(),
                        inputs: Punctuated::from_iter(inputs),
                        output: ReturnType::Type(Default::default(), Box::new(output)),
                    },
                ),
            }]),
        },
    }
}

#[inline]
pub fn fnonce_bound(inputs: impl IntoIterator<Item = Type>, output: Type) -> TraitBound {
    function_bound("FnOnce", inputs, output)
}
#[inline]
pub fn fn_bound(inputs: impl IntoIterator<Item = Type>, output: Type) -> TraitBound {
    function_bound("Fn", inputs, output)
}
#[inline]
pub fn fnmut_bound(inputs: impl IntoIterator<Item = Type>, output: Type) -> TraitBound {
    function_bound("FnMut", inputs, output)
}
