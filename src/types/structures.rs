use std::collections::HashMap;

use indexmap::IndexMap;

use super::{variant::GodotVariant, TYPE_PADDING};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GodotVector2 {
    pub x: f32,
    pub y: f32,
}

impl GodotVector2 {
    const BIT_SIZE: usize = 8;
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl GodotVariant for GodotVector2 {
    fn byte_length(&self) -> usize {
        TYPE_PADDING as usize + Self::BIT_SIZE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotVector2>() {
            self.x == other.x && self.y == other.y
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        format!("{}{}", self.x, self.y).as_bytes().to_vec()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GodotVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GodotVector3 {
    const BIT_SIZE: usize = 12;

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl GodotVariant for GodotVector3 {
    fn byte_length(&self) -> usize {
        TYPE_PADDING as usize + Self::BIT_SIZE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotVector3>() {
            self.x == other.x && self.y == other.y && self.z == other.z
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        format!("{}{}{}", self.x, self.y, self.y)
            .as_bytes()
            .to_vec()
    }
}

#[derive(Debug)]
pub struct GodotDictionary {
    pub map: IndexMap<Box<dyn GodotVariant>, Box<dyn GodotVariant>>,
    pub byte_size: usize,
}

impl GodotDictionary {
    pub fn new_from_map(map: IndexMap<Box<dyn GodotVariant>, Box<dyn GodotVariant>>) -> Self {
        Self { map, byte_size: 0 }
    }
}

impl GodotVariant for GodotDictionary {
    fn byte_length(&self) -> usize {
        self.byte_size
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<GodotDictionary>() {
            for (key, value) in self.map.iter() {
                for (okey, ovalue) in other.map.iter() {
                    if key != okey || value == ovalue {
                        return false;
                    }
                }
            }

            true
        } else {
            false
        }
    }

    fn bytes(&self) -> Vec<u8> {
        format!("{:?}", self.map).as_bytes().to_vec()
    }
}
