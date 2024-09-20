# Functions

The `fn` keywords allows you to declare new functions.

Rust uses `snake_case` as the conventional style for function and variable names:

```rust
fn main() {
    println!("Hello, world!");

    another_function() {
        println!("Another function.");
    }
}
```

Rust doesn't care where you define functions, only that they're defined somewhere in a scope that can be seen by the caller.

## Parameters

Functions can be defined to have *parameters*, which are special variables that are part of a function's signature.

You provide the *paramaters* specified by the function as *arguments*, which are the concrete values fed into the function.

In function signatures, you *must* declare the type of each parameter:

```rust
fn print_x(x: i32) {
    println!("The value of x is: {x}");
}
```

If you called `print_ln(64)`, the output would be: `The value of x is: 64`.

When defining multiple parameters, separate the parameter declarations with commas:

```rust
fn print_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression. Rust is an expression-based language, and the distinction between statements and expressions is important to understand.

* **Statements** - instructions that perform some action and do not return a value.
* **Expressions** - evaluate to a resultant value.

Creating a variable and assigning a value to it with the `let` keyword is a **statement**:

```rust
fn main() {
    let y = 6;
}
```

Function definitions are also statements; the entire code block above is a statement.

Statements do not return values. You cannot assign a `let` statement to another variable:

```rust
fn main() {
    /*
        error: expected expression, found `let` statement
        warning: unnecesary parenthesis around assigned value
    */
    let x = (let y = 6);
}
```

The statement `let y = 6` does not return a value, so there isn't anything for `x` to bind to. In other languages, such as C and Ruby, the assignment returns the value of the assignment. In those languages, you can write `x = y = 6` and have both `x` and `y` have the value `6`; that is not the case in Rust.

Expressions evaluate to a value and make up most of the rest of the code that you'll write in Rust.

Consider a math operation, such as `5 + 6`, which is an expression that evalutes to the value `11`. Expressions can be part of statements. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

```rust
fn main() {
    // assignment by expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Note that the `x + 1` does not have a semicolon at the end. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

## Functions with Return Values

Functions can return values to the code that calls them. Their type must be delcared after an arrow. The return value of the function is synonymous with the value of the final expresion in the block of the body of the function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression explicitly.

```rust
fn() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of five() is: {x}");
}
```