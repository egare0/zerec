//! `#[derive(Decode)]` code generation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericParam, Generics, Result};

use crate::attr::FieldAttrs;

/// Adds `zerec::Decode` bound to every type parameter.
fn add_decode_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut t) = *param {
            t.bounds.push(syn::parse_quote!(zerec::Decode));
        }
    }
    generics
}

pub fn derive_decode(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let generics = add_decode_bounds(input.generics.clone());
    let (impl_g, ty_g, where_g) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(s) => {
            let ctor = decode_ctor(&s.fields)?;
            quote! { Ok(#name #ctor) }
        }
        Data::Enum(e) => decode_enum(name, e)?,
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(name, "zerec does not support unions"));
        }
    };

    Ok(quote! {
        impl #impl_g zerec::Decode for #name #ty_g #where_g {
            fn decode(dec: &mut zerec::decoder::BufDecoder<'_>)
                -> Result<Self, zerec::error::DecodeError>
            {
                #body
            }
        }
    })
}

// ── helpers ───────────────────────────────────────────────────────────────

/// Generates the constructor expression for a set of fields.
fn decode_ctor(fields: &Fields) -> Result<TokenStream> {
    match fields {
        Fields::Named(f) => {
            let inits: Vec<TokenStream> = f.named.iter().map(|field| {
                let attrs = FieldAttrs::from_attrs(&field.attrs)?;
                let fname = field.ident.as_ref().unwrap();
                if attrs.skip {
                    return Ok(quote! { #fname: Default::default() });
                }
                if let Some(expr) = attrs.map_dec {
                    let closure: syn::Expr = syn::parse_str(&expr)?;
                    return Ok(quote! { #fname: { let __f = #closure; __f(dec)? } });
                }
                if let Some(via) = attrs.via {
                    let ty: syn::Type = syn::parse_str(&via)?;
                    return Ok(quote! {
                        #fname: <#ty as zerec::Adapter<_>>::decode_via(dec)?
                    });
                }
                Ok(quote! { #fname: zerec::Decode::decode(dec)? })
            }).collect::<Result<_>>()?;
            Ok(quote! { { #(#inits,)* } })
        }
        Fields::Unnamed(f) => {
            let inits: Vec<TokenStream> = f.unnamed.iter().map(|field| {
                let attrs = FieldAttrs::from_attrs(&field.attrs)?;
                if attrs.skip { return Ok(quote! { Default::default() }); }
                Ok(quote! { zerec::Decode::decode(dec)? })
            }).collect::<Result<_>>()?;
            Ok(quote! { (#(#inits,)*) })
        }
        Fields::Unit => Ok(quote! {}),
    }
}

fn decode_enum(name: &syn::Ident, data: &syn::DataEnum) -> Result<TokenStream> {
    let arms: Vec<TokenStream> = data.variants.iter().enumerate().map(|(idx, var)| {
        let vidx  = idx as u32;
        let vname = &var.ident;
        let ctor  = decode_ctor(&var.fields)?;
        Ok(quote! { #vidx => #name::#vname #ctor })
    }).collect::<Result<_>>()?;

    Ok(quote! {
        let __v = dec.read_u32()?;
        Ok(match __v {
            #(#arms,)*
            _ => return Err(zerec::error::DecodeError::UnknownVariant(__v)),
        })
    })
}