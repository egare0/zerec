//! Round-trip tests for the core codec without derive.

use zerec::{
    codec::{from_bytes, to_bytes},
    decoder::BufDecoder,
    ZeroBuf
};

// ── primitives ────────────────────────────────────────────────────────────

#[test]
fn roundtrip_u8() {
    for v in [0u8, 1, 127, 255] {
        assert_eq!(from_bytes::<u8>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_i8() {
    for v in [0i8, 1, -1, 127, -128] {
        assert_eq!(from_bytes::<i8>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_u16() {
    for v in [0u16, 1, 256, u16::MAX] {
        assert_eq!(from_bytes::<u16>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_u32() {
    for v in [0u32, 1, u32::MAX] {
        assert_eq!(from_bytes::<u32>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_u64() {
    for v in [0u64, 1, u64::MAX] {
        assert_eq!(from_bytes::<u64>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_i32() {
    for v in [0i32, 1, -1, i32::MIN, i32::MAX] {
        assert_eq!(from_bytes::<i32>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_f32() {
    for v in [0.0f32, 1.0, -1.0, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(from_bytes::<f32>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_f32_nan() {
    // NaN != NaN, check via bits
    let v = f32::NAN;
    let back = from_bytes::<f32>(&to_bytes(&v)).unwrap();
    assert!(back.is_nan());
}

#[test]
fn roundtrip_f64() {
    for v in [0.0f64, 1.0, -1.0, f64::MAX] {
        assert_eq!(from_bytes::<f64>(&to_bytes(&v)).unwrap(), v);
    }
}

#[test]
fn roundtrip_bool() {
    assert_eq!(from_bytes::<bool>(&to_bytes(&true)).unwrap(), true);
    assert_eq!(from_bytes::<bool>(&to_bytes(&false)).unwrap(), false);
}

#[test]
fn roundtrip_char() {
    for c in ['a', 'Z', '€', '🦀'] {
        assert_eq!(from_bytes::<char>(&to_bytes(&c)).unwrap(), c);
    }
}

// ── arrays and tuples ─────────────────────────────────────────────────────

#[test]
fn roundtrip_array() {
    let v: [f32; 3] = [1.0, 2.0, 3.0];
    assert_eq!(from_bytes::<[f32; 3]>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_array_zero() {
    let v: [u8; 0] = [];
    assert_eq!(from_bytes::<[u8; 0]>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_tuple() {
    let v = (1u32, true, -5i16);
    assert_eq!(from_bytes::<(u32, bool, i16)>(&to_bytes(&v)).unwrap(), v);
}

// ── collections ───────────────────────────────────────────────────────────

#[test]
fn roundtrip_vec_empty() {
    let v: Vec<u32> = vec![];
    assert_eq!(from_bytes::<Vec<u32>>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_vec() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    assert_eq!(from_bytes::<Vec<u32>>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_vec_nested() {
    let v: Vec<Vec<u8>> = vec![vec![1, 2], vec![], vec![3]];
    assert_eq!(from_bytes::<Vec<Vec<u8>>>(&to_bytes(&v)).unwrap(), v);
}

// ── string ────────────────────────────────────────────────────────────────

#[test]
fn roundtrip_string_empty() {
    let v = String::new();
    assert_eq!(from_bytes::<String>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_string() {
    let v = String::from("hello zerec");
    assert_eq!(from_bytes::<String>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_string_unicode() {
    let v = String::from("merhaba dünya 🦀");
    assert_eq!(from_bytes::<String>(&to_bytes(&v)).unwrap(), v);
}

// ── option ────────────────────────────────────────────────────────────────

#[test]
fn roundtrip_option_none() {
    let v: Option<u32> = None;
    assert_eq!(from_bytes::<Option<u32>>(&to_bytes(&v)).unwrap(), v);
}

#[test]
fn roundtrip_option_some() {
    let v: Option<u32> = Some(42);
    assert_eq!(from_bytes::<Option<u32>>(&to_bytes(&v)).unwrap(), v);
}

// ── zero-copy ─────────────────────────────────────────────────────────────

#[test]
fn zerobuf_str() {
    let original = "zero copy works";
    let bytes = to_bytes(&original);
    let mut dec = BufDecoder::new(&bytes);
    let borrowed: &str = ZeroBuf::decode_borrowed(&mut dec).unwrap();
    assert_eq!(borrowed, original);
}

#[test]
fn zerobuf_bytes() {
    let original: &[u8] = &[1, 2, 3, 4, 5];
    let bytes = to_bytes(&original.to_vec()); // Vec<u8> olarak encode et
    let mut dec = BufDecoder::new(&bytes);
    let borrowed: &[u8] = ZeroBuf::decode_borrowed(&mut dec).unwrap();
    assert_eq!(borrowed, original);
}

// ── wire format ───────────────────────────────────────────────────────────

#[test]
fn u32_is_little_endian() {
    let bytes = to_bytes(&0x01020304u32);
    assert_eq!(bytes, [0x04, 0x03, 0x02, 0x01]);
}

#[test]
fn bool_wire_values() {
    assert_eq!(to_bytes(&false), [0x00]);
    assert_eq!(to_bytes(&true), [0x01]);
}

#[test]
fn vec_length_prefix() {
    let v: Vec<u8> = vec![10, 20, 30];
    let bytes = to_bytes(&v);
    // first 4 bytes = u32 length = 3
    assert_eq!(&bytes[..4], &3u32.to_le_bytes());
    assert_eq!(&bytes[4..], &[10, 20, 30]);
}

// ── error cases ───────────────────────────────────────────────────────────

#[test]
fn unexpected_eof() {
    let result = from_bytes::<u32>(&[0x01, 0x02]); // 2 bytes, needs 4
    assert!(matches!(result, Err(zerec::DecodeError::UnexpectedEof { .. })));
}

#[test]
fn invalid_bool() {
    let result = from_bytes::<bool>(&[0x02]);
    assert!(matches!(result, Err(zerec::DecodeError::InvalidBool(2))));
}

#[test]
fn invalid_utf8() {
    // length prefix = 2, then invalid utf-8
    let mut bytes = 2u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&[0xFF, 0xFE]);
    let result = from_bytes::<String>(&bytes);
    assert!(matches!(result, Err(zerec::DecodeError::InvalidUtf8)));
}

// ── depth limit ───────────────────────────────────────────────────────────

#[test]
fn decoder_enter_at_limit() {
    let mut dec = zerec::decoder::BufDecoder::new(&[]);
    for _ in 0..64 {
        dec.enter().expect("should succeed within limit");
    }
    assert!(matches!(dec.enter(), Err(zerec::DecodeError::NestingTooDeep)));
}

#[test]
fn decoder_leave_restores_depth() {
    let mut dec = zerec::decoder::BufDecoder::new(&[]);
    for _ in 0..64 {
        dec.enter().unwrap();
    }

    assert!(dec.enter().is_err());

    dec.leave();
    dec.enter().expect("should succeed after leave");
}

#[test]
fn roundtrip_vec_triple_nested() {
    let v: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![1, 2, 3], vec![4]],
        vec![vec![5, 6]],
    ];
    assert_eq!(from_bytes::<Vec<Vec<Vec<u8>>>>(&to_bytes(&v)).unwrap(), v);
}