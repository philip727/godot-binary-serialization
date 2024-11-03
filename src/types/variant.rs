use std::{fmt::Debug, hash::Hash};

pub trait GodotVariant: Debug + Send + Sync {
    fn byte_length(&self) -> usize;

    fn as_any(&self) -> &dyn std::any::Any;

    fn variant_eq(&self, other: &dyn GodotVariant) -> bool;

    fn bytes(&self) -> Vec<u8>;
}

impl PartialEq for dyn GodotVariant {
    fn eq(&self, other: &Self) -> bool {
        self.variant_eq(other)
    }
}

impl Eq for dyn GodotVariant {}

impl Hash for Box<dyn GodotVariant> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes().hash(state)
    }
}
