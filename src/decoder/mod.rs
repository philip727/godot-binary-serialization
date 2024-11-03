pub mod dictionary;
pub mod float;
pub mod int;
pub mod string;
pub mod vector;
pub mod bool;

use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotNull, variant::GodotVariant, EncodeFlag, GodotTypeIndex};

pub struct Decoder;

impl Decoder {
    pub fn get_type_and_flags(bytes: &[u8]) -> anyhow::Result<(GodotTypeIndex, EncodeFlag)> {
        let Ok(type_idx) = GodotTypeIndex::try_from(LittleEndian::read_u16(&bytes[0..2])) else {
            return Err(anyhow!("Unsupported type index"));
        };
        let flag =
            EncodeFlag::try_from(LittleEndian::read_u16(&bytes[2..4])).unwrap_or(EncodeFlag::None);

        Ok((type_idx, flag))
    }

    pub fn decode_variant(bytes: &[u8]) -> anyhow::Result<Box<dyn GodotVariant + 'static>> {
        let (type_idx, flag) = Self::get_type_and_flags(bytes)?;

        let variant: Box<dyn GodotVariant> = match type_idx {
            GodotTypeIndex::Null => Box::new(GodotNull),
            GodotTypeIndex::Bool => Box::new(Self::decode_bool(bytes, &flag)?),
            GodotTypeIndex::Integer => Box::new(Self::decode_int(bytes, &flag)?),
            GodotTypeIndex::Float => Box::new(Self::decode_float(bytes, &flag)?),
            GodotTypeIndex::String => Box::new(Self::decode_string(bytes)?),
            GodotTypeIndex::Vector2 => Box::new(Self::decode_vector2(bytes)?),
            GodotTypeIndex::Vector3 => Box::new(Self::decode_vector3(bytes)?),
            GodotTypeIndex::Dictionary => Box::new(Self::decode_dictionary(bytes)?),
            _ => return Err(anyhow!("Unsupported godot variant of type {:?}", type_idx)),
        };

        Ok(variant)
    }
}
