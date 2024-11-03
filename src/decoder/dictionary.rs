use byteorder::{ByteOrder, LittleEndian};
use indexmap::IndexMap;

use crate::types::{primitive::GodotNull, structures::GodotDictionary};

use super::Decoder;

impl Decoder {
    pub fn decode_dictionary(bytes: &[u8]) -> anyhow::Result<GodotDictionary> {
        let mut dict = GodotDictionary {
            map: IndexMap::new(),
            byte_size: 0,
        };

        let dict_length = LittleEndian::read_u32(&bytes[4..8]);

        let mut byte_pos = 8;
        for _ in 0..dict_length {
            let key = Self::decode_variant(&bytes[byte_pos..])?;
            byte_pos += key.byte_length();

            if key.as_any().is::<GodotNull>() {
                continue;
            }

            let value = Self::decode_variant(&bytes[byte_pos..])?;
            byte_pos += value.byte_length();

            dict.map.insert(key, value);
        }

        dict.byte_size = byte_pos;

        Ok(dict)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        decoder::Decoder,
        types::{primitive::GodotString, variant::GodotVariant},
    };

    #[test]
    fn decode_dictionary() {
        let bytes = [
            18, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 104, 101, 121, 0, 4, 0, 0, 0, 4, 0, 0,
            0, 105, 32, 97, 109, 4, 0, 0, 0, 3, 0, 0, 0, 97, 103, 101, 0, 2, 0, 0, 0, 58, 0, 0, 0,
            4, 0, 0, 0, 8, 0, 0, 0, 112, 111, 115, 105, 116, 105, 111, 110, 5, 0, 0, 0, 0, 96, 21,
            70, 0, 184, 150, 69,
        ];

        let dict = Decoder::decode_dictionary(&bytes).unwrap();
        let key = Box::new(GodotString::new("position")) as Box<dyn GodotVariant>;

        let value = dict.map.get(&key);

        println!("{:?}", value);
        println!("{:?}", dict);
    }

    #[test]
    fn decode_double_dictionary() {
        let bytes = [
            18, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 104, 101, 121, 0, 4, 0, 0, 0, 4, 0, 0,
            0, 105, 32, 97, 109, 4, 0, 0, 0, 3, 0, 0, 0, 97, 103, 101, 0, 2, 0, 0, 0, 58, 0, 0, 0,
            4, 0, 0, 0, 8, 0, 0, 0, 112, 111, 115, 105, 116, 105, 111, 110, 5, 0, 0, 0, 0, 96, 21,
            70, 0, 184, 150, 69, 4, 0, 0, 0, 4, 0, 0, 0, 100, 97, 116, 97, 18, 0, 0, 0, 3, 0, 0, 0,
            4, 0, 0, 0, 6, 0, 0, 0, 98, 97, 110, 97, 110, 97, 0, 0, 2, 0, 0, 0, 23, 0, 0, 0, 4, 0,
            0, 0, 5, 0, 0, 0, 115, 119, 111, 114, 100, 0, 0, 0, 2, 0, 0, 0, 42, 0, 0, 0, 4, 0, 0,
            0, 4, 0, 0, 0, 102, 105, 115, 104, 4, 0, 0, 0, 12, 0, 0, 0, 102, 105, 115, 104, 58, 58,
            98, 97, 110, 97, 110, 97,
        ];

        let dict = Decoder::decode_dictionary(&bytes).unwrap();
        let key = Box::new(GodotString::new("position")) as Box<dyn GodotVariant>;

        let value = dict.map.get(&key);

        println!("{:?}", value);
        println!("{:?}", dict);
    }
}
