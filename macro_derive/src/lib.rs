use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Lit};
use utils::{get_attr, get_attr_opt};

mod utils;

#[proc_macro_derive(GetOnceCell, attributes(config_path))]
pub fn once_cell_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|f| &f.ident);
    let field_fn_name = fields.iter().map(|field| {
        if let Some(ident) = &field.ident {
            let name = format!("get_{}", ident.to_string());
            Some(proc_macro2::Ident::new(&name, ident.span()))
        } else {
            None
        }
    });

    let field_mut_fn_name = fields.iter().map(|field| {
        if let Some(ident) = &field.ident {
            let name = format!("get_mut_{}", ident.to_string());
            Some(proc_macro2::Ident::new(&name, ident.span()))
        } else {
            None
        }
    });

    let config_path = fields
        .iter()
        .map(|f| get_attr::<Lit>(&f.attrs, "config_path"));

    let struct_name = &input.ident;

    TokenStream::from(quote! {
        impl #struct_name {
            #(
                pub fn #field_fn_name(&self) -> &::std::collections::HashMap<&'static str, &'static str> {
                    self.#field_name.get_or_init(|| ron::from_str(include_str!(#config_path)).unwrap())
                }

                pub fn #field_mut_fn_name(&mut self) -> &mut ::std::collections::HashMap<&'static str, &'static str> {
                    let _ = self.#field_fn_name();
                    self.#field_name.get_mut().unwrap()
                }
            )*
        }
    })
}

#[proc_macro_derive(EnumParser, attributes(tag, name))]
pub fn derive_enum_parser_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("expected an enum"),
    };

    let variant_name = variants.iter().filter_map(|v| {
        if v.attrs.is_empty() {
            None
        } else {
            Some(&v.ident)
        }
    });

    let variant_tag = variants
        .iter()
        .filter_map(|v| get_attr_opt::<Lit>(&v.attrs, "tag"));

    let parser_name = get_attr::<Ident>(&input.attrs, "name");

    let out_type = &input.ident;

    TokenStream::from(quote! {
        fn #parser_name(input: &str) -> nom::IResult<&str, #out_type> {
            nom::branch::alt((
                #(
                    nom::combinator::map(nom::bytes::complete::tag(#variant_tag), |_| #out_type::#variant_name),
                )*
            ))(input)
        }
    })
}

#[proc_macro_derive(ConfigurableParser, attributes(name, keyword, config))]
pub fn derive_configurable_parser_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let parser_name = get_attr::<Ident>(&input.attrs, "name");
    let config = get_attr::<Expr>(&input.attrs, "config");
    let out_type = &input.ident;

    TokenStream::from(quote! {
        fn #parser_name<'a>(input: &'a str, config: &'a crate::config::Config) -> nom::IResult<&'a str, #out_type<'a>> {
            nom::combinator::map(
                nom::branch::alt((
                    nom::sequence::delimited(nom::bytes::complete::tag("["), nom::bytes::complete::is_not("]"), nom::bytes::complete::tag("]")),
                    nom::combinator::map_opt(nom::bytes::complete::is_not(" "), |v| config.#config().get(v).copied()),
                )),
                #out_type,
            )(input)
        }
    })
}

#[proc_macro_derive(
    KeywordConfigurableParser,
    attributes(name, keyword, config, empty_args)
)]
pub fn derive_keyword_configurable_parser_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let parser_name = get_attr::<Ident>(&input.attrs, "name");
    let config = get_attr::<Expr>(&input.attrs, "config");
    let keyword = get_attr::<Lit>(&input.attrs, "keyword");
    let empty_args = get_attr::<Lit>(&input.attrs, "empty_args");

    let empty_args = match empty_args {
        Lit::Bool(b) => b.value,
        _ => panic!("expected a bool"),
    };

    let out_type = &input.ident;

    if empty_args {
        TokenStream::from(quote! {
            fn #parser_name<'a>(input: &'a str, config: &'a crate::config::Config) -> nom::IResult<&'a str, #out_type<'a>> {
                nom::branch::alt((
                    nom::sequence::preceded(
                        nom::sequence::preceded(
                            nom::bytes::complete::tag(#keyword),
                            nom::bytes::complete::tag("-")
                        ),
                        nom::combinator::map(
                            nom::branch::alt((
                                nom::sequence::delimited(nom::bytes::complete::tag("["), nom::bytes::complete::is_not("]"), nom::bytes::complete::tag("]")),
                                nom::combinator::map_opt(nom::bytes::complete::is_not(" "), |v| config.#config().get(v).copied()),
                            )),
                            #out_type,
                        )
                    ),
                    nom::combinator::map_opt(
                        nom::bytes::complete::tag(#keyword),
                        |_| config.#config().get("").map(|v| #out_type(v))
                    ),
                ))(input)
            }
        })
    } else {
        TokenStream::from(quote! {
            fn #parser_name<'a>(input: &'a str, config: &'a crate::config::Config) -> nom::IResult<&'a str, #out_type<'a>> {
                nom::sequence::preceded(
                    nom::sequence::preceded(
                        nom::bytes::complete::tag(#keyword),
                        nom::bytes::complete::tag("-")
                    ),
                    nom::combinator::map(
                        nom::branch::alt((
                            nom::sequence::delimited(nom::bytes::complete::tag("["), nom::bytes::complete::is_not("]"), nom::bytes::complete::tag("]")),
                            nom::combinator::map_opt(nom::bytes::complete::is_not(" "), |v| config.#config().get(v).copied()),
                        )),
                        #out_type,
                    )
                )(input)
            }
        })
    }
}

#[proc_macro_derive(ConfigurableEnumParser, attributes(tag, name, config))]
pub fn derive_configurable_enum_parser_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("expected an enum"),
    };

    let variant_name = variants.iter().map(|v| &v.ident);
    let variant_tag = variants.iter().map(|v| get_attr::<Lit>(&v.attrs, "tag"));

    let parser_name = get_attr::<Ident>(&input.attrs, "name");
    let config = get_attr::<Expr>(&input.attrs, "config");
    let out_type = &input.ident;

    TokenStream::from(quote! {
        fn #parser_name<'a>(input: &'a str, config: &'a crate::config::Config) -> nom::IResult<&'a str, #out_type<'a>> {
            nom::branch::alt((
                #(
                    nom::combinator::map(
                        nom::sequence::preceded(
                            nom::sequence::terminated(nom::bytes::complete::tag(#variant_tag), nom::bytes::complete::tag("-")),
                            nom::branch::alt((
                                nom::sequence::delimited(nom::bytes::complete::tag("["), nom::bytes::complete::is_not("]"), nom::bytes::complete::tag("]")),
                                nom::combinator::map_opt(nom::bytes::complete::is_not(" "), |v| config.#config().get(v).copied())
                            )),
                        ),
                        |s| #out_type::#variant_name(s)
                    ),
                )*
            ))(input)
        }
    })
}

#[proc_macro_derive(IntoDeclaration, attributes(decl))]
pub fn derive_into_declaration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let decl_name = get_attr::<Lit>(&input.attrs, "decl");

    match &input.data {
        Data::Struct(_) => {
            let out_type = &input.ident;

            TokenStream::from(quote! {
                impl<'a> crate::class::IntoDeclaration for #out_type<'a> {
                    fn to_decl(self) -> crate::class::Decl {
                        crate::class::Decl::String(format!("{}: {}", #decl_name, self.0))
                    }
                }
            })
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let out_type = &input.ident;
            let variant_name = variants.iter().map(|v| &v.ident);
            let variant_decl = variants.iter().map(|v| {
                if let Some(attr) = get_attr_opt::<Lit>(&v.attrs, "decl") {
                    attr
                } else {
                    get_attr::<Lit>(&v.attrs, "tag")
                }
            });

            TokenStream::from(quote! {
                impl crate::class::IntoDeclaration for #out_type {
                    fn to_decl(self) -> crate::class::Decl {
                        let val = match self {
                            #(
                                Self::#variant_name => #variant_decl,
                            )*
                        };

                        crate::class::Decl::String(format!("{}: {}", #decl_name, val))
                    }
                }
            })
        }
        _ => panic!("expected enum or struct"),
    }
}
