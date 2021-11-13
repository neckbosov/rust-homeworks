use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Field, GenericArgument, Lit, Meta, NestedMeta, PathArguments, PathSegment, Type};

enum FieldKind<'a> {
    Optional(&'a Type),
    Other(&'a Type),
    Vec(&'a Type, String),
}

struct FieldInfo<'a> {
    ident: &'a Ident,
    kind: FieldKind<'a>,
}

fn get_field_kind(field: &Field) -> Result<FieldKind, syn::Error> {
    let ty = &field.ty;
    let type_path = match ty {
        Type::Path(type_path) => type_path,
        other => {
            return Ok(FieldKind::Other(other));
        }
    };
    let path = &type_path.path;
    let PathSegment { ident, arguments } = if let Some(path_segment) = path.segments.first() {
        path_segment
    } else {
        return Ok(FieldKind::Other(ty));
    };

    let generic_arg = match arguments {
        PathArguments::AngleBracketed(arguments) => {
            if let Some(GenericArgument::Type(ty)) = arguments.args.first() {
                ty
            } else {
                unreachable!()
            }
        }
        _ => {
            return Ok(FieldKind::Other(ty));
        }
    };
    if ident.to_string() == "Option".to_string() {
        Ok(FieldKind::Optional(generic_arg))
    } else if ident.to_string() == "Vec".to_string() {
        if let Some(attr) = field.attrs.iter().find(|attr| {
            let segments = &attr.path.segments;
            if segments.len() == 1 {
                segments.first().unwrap().ident.to_string() == "builder".to_string()
            } else {
                false
            }
        }) {

            let meta = attr.parse_meta()?;

            let meta_list = if let Meta::List(ref meta_list) = meta {
                meta_list
            } else {
                return Err(syn::Error::new_spanned(
                    meta,
                    "Incorrent type of attribute parameters, each = <value> expected",
                ));
            };

            let nested_meta = &meta_list.nested;
            if nested_meta.len() != 1 {
                return Err(syn::Error::new_spanned(
                    nested_meta,
                    format!(
                        "Incorrent number of attribute parameters, 1 expected, {} got",
                        nested_meta.len()
                    ),
                ));
            }
            let inner_meta = match nested_meta.first().unwrap() {
                NestedMeta::Meta(meta) => meta,
                NestedMeta::Lit(_) => {
                    return Err(syn::Error::new_spanned(
                        nested_meta,
                        "Incorrent type of attribute parameters, each = <value> expected",
                    ));
                }
            };

            let name_value = if let Meta::NameValue(name_value) = inner_meta {
                name_value
            } else {
                return Err(syn::Error::new_spanned(
                    inner_meta,
                    "Incorrent type of attribute parameters, each = <value> expected",
                ));
            };
            if name_value.path.segments.len() == 1 {
                let segment = name_value.path.segments.first().unwrap();
                if !segment.arguments.is_empty() {
                    return Err(syn::Error::new_spanned(
                        &meta,
                        format!(
                            "Incorrent name of attribute parameter, `each` expected, `{}` got",
                            name_value.path.segments.len()
                        ),
                    ));
                }

                if segment.ident.to_string() != "each".to_string() {
                    return Err(syn::Error::new_spanned(
                        meta,
                        r#"expected `builder(each = "...")`"#,
                    ));
                }
                match &name_value.lit {
                    Lit::Str(setter_name_lit) => {
                        Ok(FieldKind::Vec(generic_arg, setter_name_lit.value()))
                    }
                    lit => Err(syn::Error::new(lit.span(), "Expected string literal")),
                }
            } else {
                Err(syn::Error::new(
                    meta.span(),
                    format!(
                        "Incorrent number of attribute parameters, 1 expected, {} got",
                        name_value.path.segments.len()
                    ),
                ))
            }
        } else {
            Ok(FieldKind::Other(ty))
        }
    } else {
        Ok(FieldKind::Other(ty))
    }
}

fn get_field_declaration(field_info: &FieldInfo) -> TokenStream2 {
    let ident = field_info.ident;
    let field_ident = syn::Ident::new(&format!("{}_field", ident), Span::call_site());
    let field_kind = &field_info.kind;

    match field_kind {
        FieldKind::Other(ty) => {
            quote! {
                #field_ident: std::option::Option<#ty>,

            }
        }
        FieldKind::Optional(ty) => {
            quote! {
                #field_ident: std::option::Option<#ty>,

            }
        }
        FieldKind::Vec(ty, _) => {
            quote! {
                #field_ident: std::vec::Vec<#ty>,

            }
        }
    }
}

fn get_field_setter(field_info: &FieldInfo) -> TokenStream2 {
    let ident = field_info.ident;
    let field_ident = syn::Ident::new(&format!("{}_field", ident), Span::call_site());
    let field_kind = &field_info.kind;
    match field_kind {
        FieldKind::Other(ty) => {
            quote! {
                pub fn #ident(&mut self, value: #ty) -> &mut Self {
                    self.#field_ident = std::option::Option::Some(value);
                    self
                }

            }
        }
        FieldKind::Optional(ty) => {
            quote! {
                pub fn #ident(&mut self, value: #ty) -> &mut Self {
                    self.#field_ident = std::option::Option::Some(value);
                    self
                }

            }
        }
        FieldKind::Vec(ty, setter_name) => {
            let setter_ident = syn::Ident::new(&setter_name, Span::call_site());
            quote! {
                pub fn #setter_ident(&mut self, value: #ty) -> &mut Self {
                    self.#field_ident.push(value);
                    self
                }
            }
        }
    }
}

fn get_field_initializer(field_info: &FieldInfo) -> TokenStream2 {
    let ident = field_info.ident;
    let field_ident = syn::Ident::new(&format!("{}_field", ident), Span::call_site());
    let field_kind = &field_info.kind;

    match field_kind {
        FieldKind::Other(_) => {
            quote! {
                #ident: self.#field_ident.take()?,

            }
        }
        FieldKind::Optional(_) => {
            quote! {
                #ident: self.#field_ident.take(),

            }
        }
        FieldKind::Vec(_, _) => {
            quote! {
                #ident: std::mem::replace(&mut self.#field_ident, std::vec::Vec::new()),

            }
        }
    }
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let s = match input.data {
        Data::Struct(s) => s,
        Data::Enum(_) => {
            return quote! {
                compile_error!("Not implemented for enums");
            }
            .into();
        }
        Data::Union(_) => {
            return quote! {
                compile_error!("Not implemented for unions");
            }
            .into();
        }
    };
    let fields = s.fields;
    let mut fields_info = Vec::new();
    for field in &fields {
        let ident = field.ident.as_ref().unwrap();
        let kind = match get_field_kind(field) {
            Ok(kind) => kind,
            Err(err) => {
                return err.to_compile_error().into();
            }
        };
        fields_info.push(FieldInfo { ident, kind });
    }
    let builder_fields = fields_info.iter().map(get_field_declaration);
    let builder_setters = fields_info.iter().map(get_field_setter);
    let struct_fields_initializers = fields_info.iter().map(get_field_initializer);
    TokenStream::from(quote! {
        #[derive(Default)]
        pub struct BuilderImpl {
            #(#builder_fields)*
        }
        impl BuilderImpl {
            #(#builder_setters)*
            pub fn build(&mut self) -> std::option::Option<#struct_ident> {
                Some(#struct_ident {
                    #(#struct_fields_initializers)*
                })
            }
        }

        impl ::builder::Builder for #struct_ident {
            type BuilderType = BuilderImpl;
            fn builder() -> Self::BuilderType {
                Default::default()
            }
        }
    })
}
