# Enums

## How to decide Enum over struct ?

Structured models have same methods:
- Use Enum.

Structured models sometimes have different methods:
- Use Struct.

Functions return the value inside Enum Option:
```rust
enum Option{
	Some(value),
	None
}
```
You need to make pattern matching with handle null values.
```rust
match smthng_returns() {
	Some(value) => do_smth(),
	None => do_smth()
}
```

To handle these we can use shorter way:
```rust
item.unwrap() // If it is Some, returns Some. Otherwise panics!
item.expect("There should be value") // If it is Some, returns value. If None, prints debug message and panics. Use for crash on no value.
item.unwarp_or(&placeholder) // If it is Some, returns value. If None returns provided default value. Use when makes sense to provide fallback value.
```
