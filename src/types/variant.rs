use std::{fmt::Debug, hash::Hash};

/// Describes the variant so we can send this back to godot with correct encoding
pub trait GodotVariant: Debug + Send + Sync {
    /// Describes the byte length of a variant, most primitive variants have a static byte length.
    /// However some variants like a dictionary may have dynamic sizes
    fn byte_length(&self) -> usize;

    /// Allows us to downcast ref a variant for use
    fn as_any(&self) -> &dyn std::any::Any;

    /// Checks if a variant is equal to another variant
    fn variant_eq(&self, other: &dyn GodotVariant) -> bool;

    /// The variant as a byte vector
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
