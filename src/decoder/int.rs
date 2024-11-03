use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotInteger, EncodeFlag, TYPE_PADDING};

use super::Decoder;

impl Decoder {
    pub fn decode_int(bytes: &[u8], flag: &EncodeFlag) -> anyhow::Result<GodotInteger> {
        Self::decode_raw_int(bytes, 4, flag)
    }

    pub fn decode_raw_int(
        bytes: &[u8],
        offset: usize,
        flag: &EncodeFlag,
    ) -> anyhow::Result<GodotInteger> {
        let mut length = 4;

        if flag == &EncodeFlag::Bit64 {
            length = 8;
        }

        if bytes.len() < TYPE_PADDING as usize + length {
            return Err(anyhow!(
                "Byte slice too short to decode int with flag {flag:?}"
            ));
        }

        if flag == &EncodeFlag::Bit64 {
            return Ok(GodotInteger {
                value: LittleEndian::read_i64(&bytes[offset..offset + length]),
                byte_size: TYPE_PADDING as usize + length,
            });
        }

        Ok(GodotInteger {
            value: LittleEndian::read_i32(&bytes[offset..offset + length]) as i64,
            byte_size: TYPE_PADDING as usize + length,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;

    #[test]
    fn decode_int32() {
        let bytes: &[u8] = &[2, 0, 0, 0, 1, 0, 0, 0];
        let (_type, flag) = Decoder::get_type_and_flags(bytes).unwrap();
        let int = Decoder::decode_int(bytes, &flag).unwrap();

        assert_eq!(
            int.value, 1,
            "Expected value of {} but got {} instead",
            1, int.value
        );
    }

    #[test]
    fn decode_int64() {
        let bytes: &[u8] = &[2, 0, 1, 0, 31, 166, 227, 165, 155, 196, 32, 0];
        let (_type, flag) = Decoder::get_type_and_flags(bytes).unwrap();
        let int = Decoder::decode_int(bytes, &flag).unwrap();
        let value = 9223372036875807_i64;

        assert_eq!(
            int.value, value,
            "Expected value of {} but got {} instead",
            value, int.value
        );
    }
}
