use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotBool, SerializeFlag, TYPE_PADDING};

use super::Decoder;

impl Decoder {
    /// Decodes bytes into a Godot bool
    pub fn decode_bool(bytes: &[u8], flag: &SerializeFlag) -> anyhow::Result<GodotBool> {
        let length = GodotBool::BIT_SIZE;
        if bytes.len() < TYPE_PADDING as usize + length {
            return Err(anyhow!(
                "Byte slice too short to decode int with flag {flag:?}"
            ));
        }

        let value = LittleEndian::read_i32(&bytes[TYPE_PADDING as usize..]) == 1;

        Ok(GodotBool { value })
    }
}
