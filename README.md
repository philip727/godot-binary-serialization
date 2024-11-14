### Godot Binary Serialization
Allows us to serialize and deserialize types based on [Godot's Binary Serialization API](https://docs.godotengine.org/en/stable/tutorials/io/binary_serialization_api.html)

## Goals
- Write server infrastructure for Godot games outside of the engine and outside of Godot's supported languages.
- Rust has an extensive library ecosystem(the <b>Cargo</b> ecosystem), allowing you to offload different tasks on an external server that may
not be possible when confined to the Godot engine.


## Why an external server?
- Gives you complete control over how the server is created, run, expanded and secured.
- Godot games are very easy to reverse. Any server code written in the engine can be easily viewed with tools
such as [gdsdecomp](https://github.com/bruvzg/gdsdecomp).
- Writing a server in languages such as Rust gives you easy access to high performance, memory safe and multi-threaded code.

## Supported types

| Type        | Encode | Decode |
|-------------|--------|--------|
| Null        |    ✅   |    ✅   |
| Bool        |    ✅   |    ✅   |
| Integer     |    ✅   |    ✅   |
| Float       |    ✅   |    ✅   |
| String      |    ✅   |    ✅   |
| Vector2     |    ✅   |    ✅   |
| Rect2       |    ❌   |    ❌   |
| Vector3     |    ✅   |    ✅   |
| Transform2d |    ❌   |    ❌   |
| Plane       |    ❌   |    ❌   |
| Quaternion  |    ❌   |    ❌   |
| AABB        |    ❌   |    ❌   |
| Basis       |    ❌   |    ❌   |
| Transform3d |    ❌   |    ❌   |
| Color       |    ❌   |    ❌   |
| Node Path   |    ❌   |    ❌   |
| RID         |    ❌   |    ❌   |
| Object      |    ❌   |    ❌   |
| Dictionary  |    ✅   |    ✅   |
| Array       |    ❌   |    ❌   |
| Raw Array   |    ❌   |    ❌   |

## Examples

Decoding bytes received from a client
```rs
// Typically this would be recieved from the UDP socket and serialized in Godot with "var2bytes"
let bytes = /* Pretend this is some form of valid bytes we have received from godot */

// And it's just this simple, all you have to do is call this and it will determine
// the type and the value of that type
let variant = Decoder::decode_variant(&bytes);

// You must know the type that has been received from Godot. In this example we know that
// the client has sent us a string
let Some(string) = variant.as_any().downcast_ref::<GodotString>() else {
    panic!("DIDNT RECIEVE A STRING");
};

// Now to use the value all we have to do is access the value field
println!("{}", string.value);
```
___

Sending a string to the client
```rs
// Most Godot variants will have a new type that takes in a primitive rust value
let string = GodotString::new("hello");

// To encode the type into bytes we simply just call this and the encoder will encode it into bytes
// This will only fail if the type is unsupported or for some reason we cant write bytes to the buffer. It can only take in anything that impl GodotVariant
let Ok(bytes) = Encoder::encode_variant(string) else {
    panic!("Failed to encode variant")
};

// Assuming we have some form of UDP socket or server set up we can do something along these lines.
// And this will broadcast to everything client with the string hello
socket.broadcast(bytes, PacketDelivery::Reliable);
```
___

Sending a dictionary
```rs
// Dictionaries use the indexmap crate due to the nature that key:value pairs keep their inserted position in Godot
use indexmap::IndexMap;

// Dictionaries require a lot more boiler plate due to Godot's type system. Dictionaries can consist of
// any variant in the key or value slot, regardless of other types in the dictionary
let mut hashmap = IndexMap::new();
// Creates a key value pair with a string key of "position" and a value of Vector3
hashmap.insert(
    /// Due to the size of the type being unknown we must put the variant on the heap
    Box::new(GodotString::new("position")) as Box<dyn GodotVariant>,
    // Variants must be cast "as Box<dyn GodotVariant>" for this to work
    Box::new(GodotVector3::new(0.52, 502.0, 68.0)) as Box<dyn GodotVariant>,
);
hashmap.insert(
    Box::new(GodotString::new("id")) as Box<dyn GodotVariant>,
    Box::new(GodotInteger::new_from_i32(693)) as Box<dyn GodotVariant>,
);

// We can now create the dictionary from an index map
// Godot Dictionary structs contain a field called "byte_size" this is not needed unless decoding
// so this call just fills it in as 0
let dictionary = GodotDictionary::new_from_map(hashmap);

// We can just call this function and the encoder will do turn it into bytes
// This will only fail if the type is unsupported or for some reason we cant write bytes to the buffer. It can only take in anything that impl GodotVariant
let Ok(bytes) = Encoder::encode_variant(string) else {
    panic!("Failed to encode variant")
};

// Assuming we have some form of UDP socket or server set up we can do something along these lines.
// And this will broadcast to everything client with the dictionary
socket.broadcast(bytes, PacketDelivery::Unreliable);
```
___

Receiving a dictionary and using it's values
```rs
// Typically this would be recieved from the UDP socket and serialized in Godot with "var2bytes"
let bytes = /* Pretend we have valid bytes here */

// You must know the type that has been received from Godot. In this example we know that
// the client has sent us a dictionary
let Ok(dictionary) = Decoder::decode_variant(&bytes) else {
    panic!("Invalid bytes");
}

// Assuming the dictionary has a key value pair that has a key of "position"
let key = Box::new(GodotString::new("position")) as Box<dyn GodotVariant>;

// We can get the value now with this key
let Some(value) = dictionary.map.get(&key) else {
    panic!("Value not in dictionary");
};

// To get the value of the type we must downcast_ref it
if let Some(vector3) = value.as_any().downcast_ref::<GodotVector3>() {
    let x = vector3.x;
    let y = vector3.y;
    let z = vector3.z;
    println!("BANANA AT ({}, {}, {}) AAAAAA", x, y, z);
}
```
