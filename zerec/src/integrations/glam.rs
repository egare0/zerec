//! [`Encode`] + [`Decode`] for [glam](https://docs.rs/glam) math types.
//!
//! Enabled with `features = ["glam"]`. All types are stored as a flat
//! sequence of `f32` values in component order, no length prefix.
//!
//! | Type   | Bytes |
//! |--------|-------|
//! | `Vec2` | 8     |
//! | `Vec3` | 12    |
//! | `Vec4` | 16    |
//! | `Quat` | 16    |
//! | `Mat4` | 64 (column-major) |

use crate::{
    codec::{Decode, Encode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError,
};

impl Encode for glam::Vec2 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        self.x.encode(enc);
        self.y.encode(enc);
    }
}

impl Decode for glam::Vec2 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        Ok(Self::new(f32::decode(dec)?, f32::decode(dec)?))
    }
}

impl Encode for glam::Vec3 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        self.x.encode(enc);
        self.y.encode(enc);
        self.z.encode(enc);
    }
}

impl Decode for glam::Vec3 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        Ok(Self::new(f32::decode(dec)?, f32::decode(dec)?, f32::decode(dec)?))
    }
}

impl Encode for glam::Vec4 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        self.x.encode(enc);
        self.y.encode(enc);
        self.z.encode(enc);
        self.w.encode(enc);
    }
}

impl Decode for glam::Vec4 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        Ok(Self::new(
            f32::decode(dec)?, f32::decode(dec)?,
            f32::decode(dec)?, f32::decode(dec)?,
        ))
    }
}

impl Encode for glam::Quat {
    /// Stored as `x, y, z, w` (matches glam's internal layout).
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        self.x.encode(enc);
        self.y.encode(enc);
        self.z.encode(enc);
        self.w.encode(enc);
    }
}

impl Decode for glam::Quat {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        Ok(Self::from_xyzw(
            f32::decode(dec)?, f32::decode(dec)?,
            f32::decode(dec)?, f32::decode(dec)?,
        ))
    }
}

impl Encode for glam::Mat4 {
    /// Stored column-major: x_axis, y_axis, z_axis, w_axis (16 x f32 = 64 bytes).
    fn encode(&self, enc: &mut BufEncoder) {
        self.x_axis.encode(enc);
        self.y_axis.encode(enc);
        self.z_axis.encode(enc);
        self.w_axis.encode(enc);
    }
}

impl Decode for glam::Mat4 {
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        Ok(Self::from_cols(
            glam::Vec4::decode(dec)?,
            glam::Vec4::decode(dec)?,
            glam::Vec4::decode(dec)?,
            glam::Vec4::decode(dec)?,
        ))
    }
}