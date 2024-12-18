use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotInteger, SerializeFlag, GodotTypeIndex};

use super::Encoder;

impl Encoder {
    /// Encodes a Godot integer into bytes. A Godot integer will be encoded into its respective
    /// sizes based on the integer. If the value is over the [32 bit max value](i32::MAX) it will
    /// be encoded as a 64 bit integer
    pub fn encode_int(int: &GodotInteger) -> anyhow::Result<Vec<u8>> {
        if int.value > i32::MAX as i64 || int.value < i32::MIN as i64 {
            return Ok(Self::encode_int64(int.value));
        }

        Ok(Self::encode_int32(int.value as i32))
    }

    /// Encodes a 32 bit integer into bytes
    pub fn encode_int32(i: i32) -> Vec<u8> {
        let bytes: &mut [u8] = &mut [0; 8];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Integer as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::None as i16);
        LittleEndian::write_i32(&mut bytes[4..8], i);

        bytes.to_vec()
    }

    /// Encodes a 64 bit integer into bytes
    pub fn encode_int64(i: i64) -> Vec<u8> {
        let bytes: &mut [u8] = &mut [0; 12];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Integer as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::Bit64 as i16);
        LittleEndian::write_i64(&mut bytes[4..12], i);

        bytes.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::{encoder::Encoder, types::primitive::GodotInteger};

    #[test]
    fn encode_int32() {
        let expected_bytes = [2, 0, 0, 0, 80, 2, 0, 0].to_vec();
        let value = GodotInteger::new_from_i32(592);
        let bytes = Encoder::encode_int(&value).unwrap();

        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }

    #[test]
    fn encode_int64() {
        let expected_bytes = [2, 0, 1, 0, 107, 27, 152, 164, 225, 53, 0, 0].to_vec();
        let value = GodotInteger::new_from_i64(59243245345643);
        let bytes = Encoder::encode_int(&value).unwrap();

        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }
}
