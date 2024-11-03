use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotFloat, EncodeFlag, TYPE_PADDING};

use super::Decoder;

impl Decoder {
    pub fn decode_float(bytes: &[u8], flag: &EncodeFlag) -> anyhow::Result<GodotFloat> {
        Self::decode_raw_float(bytes, 4, flag)
    }

    pub fn decode_raw_float(
        bytes: &[u8],
        offset: usize,
        flag: &EncodeFlag,
    ) -> anyhow::Result<GodotFloat> {
        let mut length = 4;

        if flag == &EncodeFlag::Bit64 {
            length = 8;
        }

        if bytes.len() < TYPE_PADDING as usize + length {
            return Err(anyhow!(
                "Byte slice too short to decode float with flag {flag:?}"
            ));
        }

        if flag == &EncodeFlag::Bit64 {
            return Ok(GodotFloat {
                value: LittleEndian::read_f64(&bytes[offset..offset + length]),
                byte_size: TYPE_PADDING as usize + length,
            });
        }

        Ok(GodotFloat {
            value: LittleEndian::read_f32(&bytes[offset..offset + length]) as f64,
            byte_size: TYPE_PADDING as usize + length,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;

    #[test]
    fn decode_float64() {
        let bytes: &[u8] = &[3, 0, 1, 0, 123, 20, 174, 71, 225, 122, 228, 63];
        let (_type, flag) = Decoder::get_type_and_flags(bytes).unwrap();
        let float = Decoder::decode_float(bytes, &flag).unwrap();
        let value = 0.64;

        assert_eq!(
            float.value, value,
            "Expected value of {} but got {} instead",
            value, float.value
        );
    }
}
