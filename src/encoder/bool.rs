use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotBool, SerializeFlag, GodotTypeIndex};

use super::Encoder;

impl Encoder {
    /// Encodes a Godot bool into bytes
    pub fn encode_bool(r#bool: &GodotBool) -> anyhow::Result<Vec<u8>> {
        Ok(Self::encode_raw_bool(r#bool.value))
    }

    /// Encodes a bool into bytes
    pub fn encode_raw_bool(r#bool: bool) -> Vec<u8> {
        let bytes: &mut [u8] = &mut [0; 8];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Bool as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::None as i16);
        LittleEndian::write_i32(&mut bytes[4..8], r#bool as i32);

        bytes.to_vec()
    }
}
