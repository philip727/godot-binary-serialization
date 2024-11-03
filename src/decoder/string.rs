use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::primitive::GodotString;

use super::Decoder;

impl Decoder {
    pub fn decode_string(bytes: &[u8]) -> anyhow::Result<GodotString> {
        if bytes.len() < 8 {
            return Err(anyhow!("Not enough bytes for a string"));
        }

        let length = LittleEndian::read_u32(&bytes[4..8]) as usize;
        // Pad 4 bytes because godot
        let pad = (4 - (length % 4)) % 4;

        let total_length = 8 + length + pad;
        if bytes.len() < total_length {
            return Err(anyhow!("Amount of bytes does not match string length"));
        }

        let string = String::from_utf8(bytes[8..8 + length].to_vec())?;

        Ok(GodotString {
            value: string,
            byte_size: total_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;

    #[test]
    fn decode_string() {
        let bytes: &[u8] = &[
            4, 0, 0, 0, 10, 0, 0, 0, 98, 97, 110, 97, 110, 97, 66, 108, 111, 120, 0, 0,
        ];
        let (_type, _flag) = Decoder::get_type_and_flags(bytes).unwrap();
        let string = Decoder::decode_string(bytes).unwrap();
        let value = "bananaBlox";

        assert_eq!(
            string.value, value,
            "Expected value of {} but got {} instead",
            value, string.value
        )
    }
}
