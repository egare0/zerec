//! `#[derive(Encode)]` code generation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericParam, Generics, Result};

use crate::attr::FieldAttrs;

/// Adds `zerec::Encode` bound to every type parameter.
fn add_encode_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut t) = *param {
            t.bounds.push(syn::parse_quote!(zerec::Encode));
        }
    }
    generics
}

pub fn derive_encode(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let generics = add_encode_bounds(input.generics.clone());
    let (impl_g, ty_g, where_g) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(s) => encode_struct(name, &s.fields)?,
        Data::Enum(e)   => encode_enum(name, e)?,
        Data::Union(_)  => {
            return Err(syn::Error::new_spanned(name, "zerec does not support unions"));
        }
    };

    Ok(quote! {
        impl #impl_g zerec::Encode for #name #ty_g #where_g {
            fn encode(&self, enc: &mut zerec::encoder::BufEncoder) {
                #body
            }
        }
    })
}

// ── struct ────────────────────────────────────────────────────────────────

fn encode_struct(_name: &syn::Ident, fields: &Fields) -> Result<TokenStream> {
    match fields {
        Fields::Named(f) => {
            let stmts: Vec<TokenStream> = f.named.iter().map(|field| {
                let attrs = FieldAttrs::from_attrs(&field.attrs)?;
                let fname = field.ident.as_ref().unwrap();
                if attrs.skip { return Ok(quote! {}); }
                if let Some(expr) = attrs.map_enc {
                    let closure: syn::Expr = syn::parse_str(&expr)?;
                    return Ok(quote! {
                        { let __f = #closure; __f(&self.#fname).encode(enc); }
                    });
                }
                if let Some(via) = attrs.via {
                    let ty: syn::Type = syn::parse_str(&via)?;
                    return Ok(quote! {
                        <#ty as zerec::Adapter<_>>::encode_via(&self.#fname, enc);
                    });
                }
                Ok(quote! { self.#fname.encode(enc); })
            }).collect::<Result<_>>()?;
            Ok(quote! { #(#stmts)* })
        }
        Fields::Unnamed(f) => {
            let stmts: Vec<TokenStream> = f.unnamed.iter().enumerate().map(|(i, field)| {
                let attrs = FieldAttrs::from_attrs(&field.attrs)?;
                let idx = syn::Index::from(i);
                if attrs.skip { return Ok(quote! {}); }
                Ok(quote! { self.#idx.encode(enc); })
            }).collect::<Result<_>>()?;
            Ok(quote! { #(#stmts)* })
        }
        Fields::Unit => Ok(quote! {}),
    }
}

// ── enum ──────────────────────────────────────────────────────────────────

fn encode_enum(name: &syn::Ident, data: &syn::DataEnum) -> Result<TokenStream> {
    let arms: Vec<TokenStream> = data.variants.iter().enumerate().map(|(idx, var)| {
        let vidx  = idx as u32;
        let vname = &var.ident;
        match &var.fields {
            Fields::Named(f) => {
                let field_names: Vec<_> = f.named.iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                let stmts: Vec<TokenStream> = f.named.iter().map(|field| {
                    let attrs = FieldAttrs::from_attrs(&field.attrs)?;
                    let fname = field.ident.as_ref().unwrap();
                    if attrs.skip { return Ok(quote! {}); }
                    Ok(quote! { #fname.encode(enc); })
                }).collect::<Result<_>>()?;
                Ok(quote! {
                    #name::#vname { #(#field_names),* } => {
                        enc.write_u32(#vidx);
                        #(#stmts)*
                    }
                })
            }
            Fields::Unnamed(f) => {
                let bindings: Vec<_> = (0..f.unnamed.len())
                    .map(|i| quote::format_ident!("_f{i}"))
                    .collect();
                let stmts: Vec<TokenStream> = bindings.iter()
                    .map(|b| quote! { #b.encode(enc); })
                    .collect();
                Ok(quote! {
                    #name::#vname(#(#bindings),*) => {
                        enc.write_u32(#vidx);
                        #(#stmts)*
                    }
                })
            }
            Fields::Unit => Ok(quote! {
                #name::#vname => { enc.write_u32(#vidx); }
            }),
        }
    }).collect::<Result<_>>()?;

    Ok(quote! { match self { #(#arms)* } })
}