use byteorder::{LittleEndian, WriteBytesExt};

use crate::types::{structures::GodotDictionary, SerializeFlag, GodotTypeIndex};

use super::Encoder;

impl Encoder {
    /// Encodes a Godot dictionary into bytes. A godot dictionary consists of key value pairs.
    /// 
    /// # Example
    /// 
    /// Due to the nature of Godot's type system, key's and value's can be different types. Due to
    /// the nature of Rust, this adds quite a bit of overhead to creating a Dictionary
    /// ```json
    /// { "key": "value", "key2": 42, Vector3(45, 2, 9): 9529 }
    /// ```
    pub fn encode_dictionary(dictionary: &GodotDictionary) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();

        let iterator = dictionary.map.iter();
        let length = iterator.len();

        bytes.write_i16::<LittleEndian>(GodotTypeIndex::Dictionary as i16)?;
        bytes.write_i16::<LittleEndian>(SerializeFlag::None as i16)?;
        bytes.write_i32::<LittleEndian>(length as i32)?;

        for (key, value) in iterator {
            let mut key_bytes = Encoder::encode_variant(&**key)?;
            bytes.append(&mut key_bytes);
            let mut value_bytes = Encoder::encode_variant(&**value)?;
            bytes.append(&mut value_bytes);
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        encoder::Encoder,
        types::{
            primitive::{GodotInteger, GodotString},
            structures::{GodotDictionary, GodotVector3},
        },
    };

    #[test]
    fn encode_dictionary() {
        let expected_bytes = [
            18, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0, 112, 111, 115, 105, 116, 105, 111,
            110, 7, 0, 0, 0, 184, 30, 5, 63, 0, 0, 251, 67, 0, 0, 136, 66, 4, 0, 0, 0, 2, 0, 0, 0,
            105, 100, 0, 0, 2, 0, 0, 0, 181, 2, 0, 0,
        ]
        .to_vec();

        let mut dict = GodotDictionary::new();
        dict.insert(GodotString::new("position"), GodotVector3::new(0.52, 502.0, 68.0));
        dict.insert(GodotString::new("id"), GodotInteger::new_from_i32(693));

        let bytes = Encoder::encode_dictionary(&dict).unwrap();
        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }
}
