use core::f32;

use byteorder::{ByteOrder, LittleEndian};

use crate::types::{
    primitive::GodotFloat, variant::GodotVariant, SerializeFlag, GodotTypeIndex, TYPE_PADDING,
};

use super::Encoder;

impl Encoder {
    /// Encodes a Godot float into bytes. A Godot float will be encoded into its respective
    /// sizes based on the float. If the byte length of the float is over the 
    /// [32 bit size](GodotFloat::BIT_32_SIZE)
    pub fn encode_float(float: &GodotFloat) -> anyhow::Result<Vec<u8>> {
        if float.byte_length() > TYPE_PADDING as usize + GodotFloat::BIT_32_SIZE {
            return Ok(Self::encode_f64(float.value));
        }

        Ok(Self::encode_f32(float.value as f32))
    }

    /// Encodes a 32 bit float into bytes
    pub fn encode_f32(i: f32) -> Vec<u8> {
        let bytes: &mut [u8] = &mut [0; 8];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Float as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::None as i16);
        LittleEndian::write_f32(&mut bytes[4..8], i);

        bytes.to_vec()
    }

    /// Encodes a 64 bit float into bytes
    pub fn encode_f64(i: f64) -> Vec<u8> {
        let bytes: &mut [u8] = &mut [0; 12];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Float as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::Bit64 as i16);
        LittleEndian::write_f64(&mut bytes[4..12], i);

        bytes.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::{encoder::Encoder, types::primitive::GodotFloat};

    #[test]
    fn encode_f64() {
        let expected_bytes = [3, 0, 1, 0, 174, 230, 149, 231, 52, 245, 226, 63].to_vec();
        let value = GodotFloat::new_from_f64(0.59243245345643);
        let bytes = Encoder::encode_float(&value).unwrap();

        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }
}
