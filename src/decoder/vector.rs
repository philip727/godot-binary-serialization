use crate::types::{
    structures::{GodotVector2, GodotVector3},
    SerializeFlag,
};

use super::Decoder;

impl Decoder {
    /// Decodes bytes into a vector 2. This will fail if the inner bytes can't be decoded into a
    /// float
    pub fn decode_vector2(bytes: &[u8]) -> anyhow::Result<GodotVector2> {
        let x = Decoder::decode_raw_float(bytes, 4, &SerializeFlag::None)?.value as f32;
        let y = Decoder::decode_raw_float(bytes, 8, &SerializeFlag::None)?.value as f32;

        Ok(GodotVector2 { x, y })
    }

    /// Decodes bytes into a vector 3. This will fail if the inner bytes can't be decoded into a
    /// float
    pub fn decode_vector3(bytes: &[u8]) -> anyhow::Result<GodotVector3> {
        let x = Decoder::decode_raw_float(bytes, 4, &SerializeFlag::None)?.value as f32;
        let y = Decoder::decode_raw_float(bytes, 8, &SerializeFlag::None)?.value as f32;
        let z = Decoder::decode_raw_float(bytes, 12, &SerializeFlag::None)?.value as f32;

        Ok(GodotVector3 { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        decoder::Decoder,
        types::structures::{GodotVector2, GodotVector3},
    };

    #[test]
    fn decode_vector2() {
        let bytes = [5, 0, 0, 0, 0, 0, 134, 66, 0, 31, 94, 71];
        let (_type, _flag) = Decoder::get_type_and_flags(&bytes).unwrap();
        let vector2 = Decoder::decode_vector2(&bytes).unwrap();
        let value = GodotVector2 {
            x: 67.0,
            y: 56863.0,
        };

        assert_eq!(
            vector2, value,
            "Expected value of {:?} but got {:?} instead",
            value, vector2
        );
    }

    #[test]
    fn decode_vector3() {
        let bytes = [7, 0, 0, 0, 0, 0, 134, 66, 0, 31, 94, 71, 0, 179, 168, 199];
        let (_type, _flag) = Decoder::get_type_and_flags(&bytes).unwrap();
        let vector2 = Decoder::decode_vector3(&bytes).unwrap();
        let value = GodotVector3 {
            x: 67.0,
            y: 56863.0,
            z: -86374.0,
        };

        assert_eq!(
            vector2, value,
            "Expected value of {:?} but got {:?} instead",
            value, vector2
        );
    }
}
