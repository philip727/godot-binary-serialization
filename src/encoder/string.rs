use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotString, EncodeFlag, GodotTypeIndex};

use super::Encoder;

impl Encoder {
    pub fn encode_string(string: &GodotString) -> anyhow::Result<Vec<u8>> {
        Ok(Self::encode_owned_string(string.value.clone()))
    }

    pub fn encode_owned_string(string: String) -> Vec<u8> {
        let length = string.len();
        let pad = (4 - (length % 4)) % 4;
        let total_length = 8 + length + pad;
        let mut bytes = vec![0; total_length];

        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::String as i16);
        LittleEndian::write_i16(&mut bytes[2..4], EncodeFlag::None as i16);
        LittleEndian::write_i32(&mut bytes[4..8], length as i32);
        bytes[8..8 + length].copy_from_slice(string.as_bytes());

        bytes
    }

    pub fn encode_str(string: &str) -> Vec<u8> {
        Self::encode_owned_string(string.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::{encoder::Encoder, types::primitive::GodotString};

    #[test]
    fn encode_string() {
        let expected_bytes = [
            4, 0, 0, 0, 10, 0, 0, 0, 98, 97, 110, 97, 110, 97, 66, 108, 111, 120, 0, 0,
        ]
        .to_vec();
        let value = GodotString::new("bananaBlox");
        let bytes = Encoder::encode_string(&value).unwrap();

        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }
}
