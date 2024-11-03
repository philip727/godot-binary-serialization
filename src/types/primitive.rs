use std::hash::Hash;

use super::{variant::GodotVariant, TYPE_PADDING};

#[derive(Debug)]
pub struct GodotNull;

impl GodotVariant for GodotNull {
    fn byte_length(&self) -> usize {
        TYPE_PADDING as usize
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        other.as_any().downcast_ref::<GodotNull>().is_some()
    }

    fn bytes(&self) -> Vec<u8> {
        format!("{:?}", self).as_bytes().to_vec()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GodotInteger {
    pub value: i64,
    pub byte_size: usize,
}

impl GodotInteger {
    const BIT_32_SIZE: usize = 4;
    const BIT_64_SIZE: usize = 8;
    pub fn new_from_i32(v: i32) -> Self {
        Self {
            value: v as i64,
            byte_size: TYPE_PADDING as usize + Self::BIT_32_SIZE,
        }
    }

    pub fn new_from_i64(v: i64) -> Self {
        Self {
            value: v,
            byte_size: TYPE_PADDING as usize + Self::BIT_64_SIZE,
        }
    }
}

impl GodotVariant for GodotInteger {
    fn byte_length(&self) -> usize {
        self.byte_size
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotInteger>() {
            self.value == other.value
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        self.value.to_string().as_bytes().to_vec()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GodotFloat {
    pub value: f64,
    pub byte_size: usize,
}

impl GodotFloat {
    pub const BIT_32_SIZE: usize = 4;
    pub const BIT_64_SIZE: usize = 8;
    pub fn new_from_f32(v: f32) -> Self {
        Self {
            value: v as f64,
            byte_size: TYPE_PADDING as usize + Self::BIT_32_SIZE,
        }
    }

    pub fn new_from_f64(v: f64) -> Self {
        Self {
            value: v,
            byte_size: TYPE_PADDING as usize + Self::BIT_64_SIZE,
        }
    }
}

impl GodotVariant for GodotFloat {
    fn byte_length(&self) -> usize {
        self.byte_size
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotFloat>() {
            self.value == other.value
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        self.value.to_string().as_bytes().to_vec()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct GodotString {
    pub value: String,
    pub byte_size: usize,
}

impl GodotString {
    pub fn new(s: &str) -> Self {
        let length = s.len();
        // Pad 4 bytes because godot
        let pad = (4 - (length % 4)) % 4;
        Self {
            value: s.to_owned(),
            byte_size: TYPE_PADDING as usize + pad + length,
        }
    }
}

impl GodotVariant for GodotString {
    fn byte_length(&self) -> usize {
        self.byte_size
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotString>() {
            self.value == other.value
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GodotBool {
    pub value: bool,
}

impl GodotBool {
    pub const BIT_SIZE: usize = 4;

    pub fn new(r#bool: bool) -> Self {
        Self { value: r#bool }
    }
}

impl GodotVariant for GodotBool {
    fn byte_length(&self) -> usize {
        TYPE_PADDING as usize + Self::BIT_SIZE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotBool>() {
            self.value == other.value
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        format!("{:?}", self.value).as_bytes().to_vec()
    }
}
