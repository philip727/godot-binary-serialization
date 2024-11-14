use byteorder::{ByteOrder, LittleEndian};

use crate::types::{
    structures::{GodotVector2, GodotVector3},
    SerializeFlag, GodotTypeIndex,
};

use super::Encoder;

impl Encoder {
    /// Encodes a Vector2 into bytes
    pub fn encode_vector2(vec2: &GodotVector2) -> anyhow::Result<Vec<u8>> {
        let bytes = &mut [0; 12];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Vector2 as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::None as i16);
        LittleEndian::write_f32(&mut bytes[4..8], vec2.x);
        LittleEndian::write_f32(&mut bytes[8..12], vec2.y);

        Ok(bytes.to_vec())
    }

    /// Encodes a Vector3 into bytes
    pub fn encode_vector3(vec3: &GodotVector3) -> anyhow::Result<Vec<u8>> {
        let bytes = &mut [0; 16];
        LittleEndian::write_i16(&mut bytes[0..2], GodotTypeIndex::Vector3 as i16);
        LittleEndian::write_i16(&mut bytes[2..4], SerializeFlag::None as i16);
        LittleEndian::write_f32(&mut bytes[4..8], vec3.x);
        LittleEndian::write_f32(&mut bytes[8..12], vec3.y);
        LittleEndian::write_f32(&mut bytes[12..16], vec3.z);

        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        encoder::Encoder,
        types::structures::{GodotVector2, GodotVector3},
    };

    #[test]
    fn encode_vector2() {
        let expected_bytes = [5, 0, 0, 0, 0, 0, 80, 66, 128, 162, 133, 71].to_vec();
        let value = GodotVector2::new(52.0, 68421.0);
        let bytes = Encoder::encode_vector2(&value).unwrap();

        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }

    #[test]
    fn encode_vector3() {
        let expected_bytes =
            [7, 0, 0, 0, 0, 0, 80, 66, 128, 162, 133, 71, 224, 46, 14, 73].to_vec();
        let value = GodotVector3::new(52.0, 68421.0, 582382.0);
        let bytes = Encoder::encode_vector3(&value).unwrap();

        assert_eq!(
            expected_bytes, bytes,
            "Expected {:?} but got {:?}",
            expected_bytes, bytes
        );
    }
}
