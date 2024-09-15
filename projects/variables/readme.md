# Variables and Mutability

By default, variables are immutable. This is one of many nudges Rust gives you to write code in a way that takes advantage of the safety and easy concurrency that Rust offers.

```rust
let x = 5;
/*
    error: cannot assign twice
    to immutable variable
*/
x = 6;
```

You still have the option to make your variables mutable.

```rust
let mut x = 5;
x = 6;
```

## Constants

*Constants* are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables:

* You aren't allowed to use `mut` with constants. Constant's aren't just immutable by default - they're *always* immutable. You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value *must* be annotated.

* Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.

* Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust's naming convention for constants is to use all uppercase with underscores between words.

The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that's easier to understand and verify, rather than setting this constant to the value 10,800.

> See the [Rust Reference's section on constant evaluation](https://doc.rust-lang.org/reference/const_eval.html) for more information on what operations can be used when declaring constants.

constants are valid for the entire time a program runs, within the scope in which they were declared.

Naming hardcoded values used throughout your program as constants is helpful in conveying the meaning of that value to future maintainers of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.

## Shadowing

You can declare a new variable with the same name as a previous variable. The first variables is referred to being *shadowed* by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

The second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable using the same variable's name and repeating the use of the `let` keyword as follows:

```bash
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Shadowing is different from marking a variable as `mut` because we'll get a compile-time error if we accidentally try to reassign tot his variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between `mut` shadowing is that because we're effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

For example, say our program asks a use to show how many spaces they want between some text by inputting space characters, and we want to store that input as a number:

```rust
let spaces = "    ";
let spaces = spaces.len();
```

This will not work with:

```rust
let mut spaces = "    ";

/*
    error:
    expected `&str`, found `usize`
*/
spaces = spaces.len();
```