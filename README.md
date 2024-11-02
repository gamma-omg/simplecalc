# SimpleCalc
A simple and efficient mathematical expression interpreter in Rust.

## Features
**Operators Supported**:
- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)
- Exponentiation (`**`)

## Usage
Here's a basic example of how to use SimpleCalc:
```rust
fn main() {
    match simplecalc::eval("1+2-3*4/5**6") {
        Ok(res) => println!("{res}"),
        Err(e) => println!("{e}"),
    }
}
```

Evaluate an expression:
```rust
let result = simplecalc::eval("10 + 5 * 2")?;
println!("{}", result); // Outputs: 20
```

Handle invalid expressions:
```rust
println!("{}", simplecalc::eval("invalud expression").err().unwrap());
```