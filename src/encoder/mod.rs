use anyhow::anyhow;

use crate::types::{
    primitive::{GodotBool, GodotFloat, GodotInteger, GodotString},
    structures::{GodotDictionary, GodotVector2, GodotVector3},
    variant::{AsVariant, GodotVariant},
};

pub mod dictionary;
pub mod float;
pub mod int;
pub mod string;
pub mod vector;
pub mod bool;

/// Encodes a variant from its type into bytes
pub struct Encoder;

impl Encoder {
    /// Takes in a Godot variant and determines how to encode it based on its type
    pub fn encode_variant(variant: &dyn GodotVariant) -> anyhow::Result<Vec<u8>> {
        if let Some(bool) = variant.as_var::<GodotBool>() {
            return Self::encode_bool(bool);
        }

        if let Some(integer) = variant.as_var::<GodotInteger>() {
            return Self::encode_int(integer);
        }

        if let Some(float) = variant.as_var::<GodotFloat>() {
            return Self::encode_float(float);
        }

        if let Some(string) = variant.as_var::<GodotString>() {
            return Self::encode_string(string);
        }

        if let Some(vector2) = variant.as_var::<GodotVector2>() {
            return Self::encode_vector2(vector2);
        }

        if let Some(vector3) = variant.as_var::<GodotVector3>() {
            return Self::encode_vector3(vector3);
        }

        if let Some(dictionary) = variant.as_var::<GodotDictionary>() {
            return Self::encode_dictionary(dictionary);
        }

        Err(anyhow!(
            "Variant of {:?} is not supported by the encoder",
            variant
        ))
    }
}
