//! `#[zerec(...)]` field attribute parsing.

use syn::{Attribute, LitStr, Result};

#[derive(Default)]
pub struct FieldAttrs {
    /// `#[zerec(skip)]`
    ///
    /// Exclude this field from encode/decode. The field must implement
    /// `Default` so the decoder can fill it without reading any bytes.
    pub skip: bool,

    /// `#[zerec(via = "AdapterType")]`
    ///
    /// Route encode/decode through a type that implements
    /// `zerec::Adapter<FieldType>`
    pub via: Option<String>,

    /// `#[zerec(map_enc = "|v| expr")]`
    ///
    /// A closure that converts `&FieldType` into any type that implements
    /// `Encode`. Inlined as-is into the generated code.
    pub map_enc: Option<String>,

    /// `#[zerec(map_dec = "|dec| expr")]`
    ///
    /// A closure that takes `&mut BufDecoder` and returns
    /// `Result<FieldType, DecodeError>`.
    pub map_dec: Option<String>,
}

impl FieldAttrs {
    /// Parses all `#[zerec(...)]` attributes on a field.
    ///
    /// Unknown keys produce a compile error pointing at the offending
    /// attribute so the user gets a precise diagnostic.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut out = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("zerec") { continue; }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    out.skip = true;
                } else if meta.path.is_ident("via") {
                    let v: LitStr = meta.value()?.parse()?;
                    out.via = Some(v.value());
                } else if meta.path.is_ident("map_enc") {
                    let v: LitStr = meta.value()?.parse()?;
                    out.map_enc = Some(v.value());
                } else if meta.path.is_ident("map_dec") {
                    let v: LitStr = meta.value()?.parse()?;
                    out.map_dec = Some(v.value());
                } else {
                    return Err(meta.error("unknown zerec attribute"));
                }

                Ok(())
            })?;
        }

        Ok(out)
    }
}