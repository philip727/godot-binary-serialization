pub mod dictionary;
pub mod float;
pub mod int;
pub mod string;
pub mod vector;
pub mod bool;

use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::{primitive::GodotNull, variant::GodotVariant, SerializeFlag, GodotTypeIndex};

pub struct Decoder;

impl Decoder {
    /// Gets the type and flags of the bytes passed. The type determines which type we should try
    /// and decode it as, and the flag shows how we will decode the type
    pub fn get_type_and_flags(bytes: &[u8]) -> anyhow::Result<(GodotTypeIndex, SerializeFlag)> {
        let Ok(type_idx) = GodotTypeIndex::try_from(LittleEndian::read_u16(&bytes[0..2])) else {
            return Err(anyhow!("Unsupported type index"));
        };
        let flag =
            SerializeFlag::try_from(LittleEndian::read_u16(&bytes[2..4])).unwrap_or(SerializeFlag::None);

        Ok((type_idx, flag))
    }

    /// Decodes bytes into it's respective Godot variant. This can fail if the bytes does not match
    /// Godot's serialization rules or it's an unsupported type.
    pub fn decode_variant(bytes: &[u8]) -> anyhow::Result<Box<dyn GodotVariant + 'static>> {
        if bytes.is_empty() {
            return Err(anyhow!("Empty bytes"));
        }

        let (type_idx, flag) = Self::get_type_and_flags(bytes)?;

        let variant: Box<dyn GodotVariant> = match type_idx {
            GodotTypeIndex::Nil => Box::new(GodotNull),
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
