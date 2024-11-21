# Godot Binary Serialization
Allows us to serialize and deserialize types based on [Godot's Binary Serialization API](https://docs.godotengine.org/en/stable/tutorials/io/binary_serialization_api.html)

## Getting started
Add the crate to your project from [crates.io](https://crates.io/crates/godot-binary-serialization)

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
let Some(string) = variant.as_var::<GodotString>() else {
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
// We can simply create a godot dictionary as such
let mut dict = GodotDictionary::new();

// And insert values just as if it was a hashmap. The values must impl GodotVariant and the types
// don't have to match for each key/value
dict.insert(GodotString::new("position"), GodotVector3::new(0.52, 502.0, 68.0));
dict.insert(GodotString::new("id"), GodotInteger::new_from_i32(693));

// We can just call this function and the encoder will do turn it into bytes
// This will only fail if the type is unsupported or for some reason we cant write bytes to the buffer. It can only take in anything that impl GodotVariant
let Ok(bytes) = Encoder::encode_variant(dict) else {
    panic!("Failed to encode variant")
};

// Assuming we have some form of UDP socket or server set up we can do something along these lines.
// And this will broadcast to everything client with the dictionary
socket.broadcast(bytes, PacketDelivery::Unreliable);
```
___

Recieving a dictionary and using it's values
```rs
// Typically this would be recieved from the UDP socket and serialized in Godot with "var2bytes"
let bytes = /* Pretend we have valid bytes here */

// You must know the type that has been recieved from Godot. In this example we know that
// the client has sent us a dictionary
let Ok(dictionary) = Decoder::decode_variant(&bytes) else {
    panic!("Invalid bytes");
}

// Assuming the dictionary has a key value pair that has a key of "position" with type Vector3
let Some(value) = dictionary.get::<Vector3>(GodotString::new("position")) else {
    panic!("Value not in dictionary");
};

let x = value.x;
let y = value.y;
let z = value.z;
println!("BANANA AT ({}, {}, {}) AAAAAA", x, y, z);
```
