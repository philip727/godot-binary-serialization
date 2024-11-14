pub mod variant;
pub mod primitive;
pub mod structures;

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
    Null = 0,
    Bool = 1,
    Integer = 2,
    Float = 3,
    String = 4,
    Vector2 = 5,
    Rect2 = 6,
    Vector3 = 7,
    Transform2D = 8,
    Plane = 9,
    Quaternion = 10,
    Aabb = 11,
    Basis = 12,
    Transform3D = 13,
    Color = 14,
    NodePath = 15,
    RID = 16,
    Object = 17,
    Dictionary = 18,
    Array = 19,
    RawArray = 20,
    Int32Array = 21,
    Int64Array = 22,
    Float32Array = 23,
    Float64Array = 24,
    StringArray = 25,
    Vector2Array = 26,
    Vector3Array = 27,
    ColorArray = 28,
    Max = 29,
}

impl TryFrom<u16> for GodotTypeIndex {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GodotTypeIndex::Null),
            1 => Ok(GodotTypeIndex::Bool),
            2 => Ok(GodotTypeIndex::Integer),
            3 => Ok(GodotTypeIndex::Float),
            4 => Ok(GodotTypeIndex::String),
            5 => Ok(GodotTypeIndex::Vector2),
            6 => Ok(GodotTypeIndex::Rect2),
            7 => Ok(GodotTypeIndex::Vector3),
            8 => Ok(GodotTypeIndex::Transform2D),
            9 => Ok(GodotTypeIndex::Plane),
            10 => Ok(GodotTypeIndex::Quaternion),
            11 => Ok(GodotTypeIndex::Aabb),
            12 => Ok(GodotTypeIndex::Basis),
            13 => Ok(GodotTypeIndex::Transform3D),
            14 => Ok(GodotTypeIndex::Color),
            15 => Ok(GodotTypeIndex::NodePath),
            16 => Ok(GodotTypeIndex::RID),
            17 => Ok(GodotTypeIndex::Object),
            18 => Ok(GodotTypeIndex::Dictionary),
            19 => Ok(GodotTypeIndex::Array),
            20 => Ok(GodotTypeIndex::RawArray),
            21 => Ok(GodotTypeIndex::Int32Array),
            22 => Ok(GodotTypeIndex::Int64Array),
            23 => Ok(GodotTypeIndex::Float32Array),
            24 => Ok(GodotTypeIndex::Float64Array),
            25 => Ok(GodotTypeIndex::StringArray),
            26 => Ok(GodotTypeIndex::Vector2Array),
            27 => Ok(GodotTypeIndex::Vector3Array),
            28 => Ok(GodotTypeIndex::ColorArray),
            29 => Ok(GodotTypeIndex::Max),
            _ => Err(()),
        }
    }
}
