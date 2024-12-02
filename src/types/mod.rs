pub mod primitive;
pub mod structures;
pub mod variant;

pub const TYPE_PADDING: u8 = 4;

/// The different serialization flags from Godot's binary serialization
#[derive(PartialEq, Eq, Debug)]
pub enum SerializeFlag {
    /// The encoder has no flags
    None = 0,
    /// Used for integers and floats, shows whether they are represented as a 64 bit or 32 bit
    /// value. 0 = 32 bit, 1 = 64 Bit
    Bit64 = 1,
}

impl TryFrom<u16> for SerializeFlag {
    type Error = ();

    fn try_from(value: u16) -> Result<SerializeFlag, Self::Error> {
        match value {
            0 => Ok(SerializeFlag::None),
            1 => Ok(SerializeFlag::Bit64),
            _ => Err(()),
        }
    }
}

/// The Godot type indexes based on Godot's binary serialization API
#[derive(PartialEq, Eq, Debug)]
pub enum GodotTypeIndex {
    Nil = 0,
    Bool = 1,
    Integer = 2,
    Float = 3,
    String = 4,
    Vector2 = 5,
    Vector2I = 6,
    Rect2 = 7,
    Rect2I = 8,
    Vector3 = 9,
    Vector3I = 10,
    Transform2D = 11,
    Vector4 = 12,
    Vector4I = 13,
    Plane = 14,
    Quaternion = 15,
    Aabb = 16,
    Basis = 17,
    Transform3D = 18,
    Projection = 19,
    Color = 20,
    StringName = 21,
    NodePath = 22,
    RID = 23,
    Object = 24,
    Callable = 25,
    Signal = 26,
    Dictionary = 27,
    Array = 28,
}

impl TryFrom<u16> for GodotTypeIndex {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GodotTypeIndex::Nil),
            1 => Ok(GodotTypeIndex::Bool),
            2 => Ok(GodotTypeIndex::Integer),
            3 => Ok(GodotTypeIndex::Float),
            4 => Ok(GodotTypeIndex::String),
            5 => Ok(GodotTypeIndex::Vector2),
            6 => Ok(GodotTypeIndex::Vector2I),
            7 => Ok(GodotTypeIndex::Rect2),
            8 => Ok(GodotTypeIndex::Rect2I),
            9 => Ok(GodotTypeIndex::Vector3),
            10 => Ok(GodotTypeIndex::Vector3I),
            11 => Ok(GodotTypeIndex::Transform2D),
            12 => Ok(GodotTypeIndex::Vector4),
            13 => Ok(GodotTypeIndex::Vector4I),
            14 => Ok(GodotTypeIndex::Plane),
            15 => Ok(GodotTypeIndex::Quaternion),
            16 => Ok(GodotTypeIndex::Aabb),
            17 => Ok(GodotTypeIndex::Basis),
            18 => Ok(GodotTypeIndex::Transform3D),
            19 => Ok(GodotTypeIndex::Projection),
            20 => Ok(GodotTypeIndex::Color),
            21 => Ok(GodotTypeIndex::StringName),
            22 => Ok(GodotTypeIndex::NodePath),
            23 => Ok(GodotTypeIndex::RID),
            24 => Ok(GodotTypeIndex::Object),
            25 => Ok(GodotTypeIndex::Callable),
            26 => Ok(GodotTypeIndex::Signal),
            27 => Ok(GodotTypeIndex::Dictionary),
            28 => Ok(GodotTypeIndex::Array),
            _ => Err(()),
        }
    }
}
