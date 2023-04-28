use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Expr, Fields, Lit};

#[proc_macro_derive(GetOnceCell, attributes(config_path))]
pub fn derive(input: TokenStream) -> TokenStream {
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

    let config_path = fields.iter().map(|f| {
        if let Some(attr) = f.attrs.first() {
            if attr.path().is_ident("config_path") {
                let path: Lit = attr.parse_args().unwrap();
                Some(path)
            } else {
                None
            }
        } else {
            None
        }
    });

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
