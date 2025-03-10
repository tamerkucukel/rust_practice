# Error Handling

This example is about error handling on meaningful values.
```rust
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Divide by Zero Error."))
    } else {
        Ok(a / b)
    }
}
```

```rust
match divide(5.0, 0.0) {
	Ok(result_of_division) => {
		println!("{}", result_of_division);
	}
	Err(err_message) => {
		println!("{}", err_message);
	}
}
```

This example is about error handling on no meaningful value for Ok() Result values.
- By convention, Rust wants us to use empty tuple () for returning no meaningful value on success.
- In the matching part, by writing ".." inside Ok(), we say that "Ok there is a value inside but I omit it.". Therefore case is matched and succeed value is handled.
```rust
fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("Too much ingredients."))
    } else {
        Ok(())
    }
}
```

```rust
match validate_ingredients(&ingredients) {
	Ok(..) => println!("Ingredients are ok."),
	Err(err_message) => println!("{}", err_message)
}
```

You may use expect to handle errors.

```rust
let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
let error_logs = extract_errors(text.as_str());
fs::write("errors.log", error_logs.join("\n")).expect("Failed to write errors.txt");
```

Or you can use try "?" operator to generate or propagate errors to upper scope.

```rust
fn main() -> Result<(), Error> {
	let text = fs::read_to_string("logs.txt")?;
	let error_logs = extract_errors(text.as_str());
	fs::write("errors.log", error_logs.join("\n"))?;
	Ok(())
}
```

## How to decide ?

if let, match statements
- If you have meaningful handling way for error cases.
unwrap, expect
- Quick debugging, testing purposes.
try operator
- If you don't have meaningful handling way for error cases.
