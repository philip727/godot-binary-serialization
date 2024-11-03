use anyhow::anyhow;

use crate::types::{
    primitive::{GodotFloat, GodotInteger, GodotString},
    structures::{GodotDictionary, GodotVector2, GodotVector3},
    variant::GodotVariant,
};

pub mod dictionary;
pub mod float;
pub mod int;
pub mod string;
pub mod vector;

pub struct Encoder;

impl Encoder {
    pub fn encode_variant(variant: &dyn GodotVariant) -> anyhow::Result<Vec<u8>> {
        if let Some(integer) = variant.as_any().downcast_ref::<GodotInteger>() {
            return Self::encode_int(integer);
        }

        if let Some(float) = variant.as_any().downcast_ref::<GodotFloat>() {
            return Self::encode_float(float);
        }

        if let Some(string) = variant.as_any().downcast_ref::<GodotString>() {
            return Self::encode_string(string);
        }

        if let Some(vector2) = variant.as_any().downcast_ref::<GodotVector2>() {
            return Self::encode_vector2(vector2);
        }

        if let Some(vector3) = variant.as_any().downcast_ref::<GodotVector3>() {
            return Self::encode_vector3(vector3);
        }

        if let Some(dictionary) = variant.as_any().downcast_ref::<GodotDictionary>() {
            return Self::encode_dictionary(dictionary);
        }

        Err(anyhow!(
            "Variant of {:?} is not supported by the encoder",
            variant
        ))
    }
}