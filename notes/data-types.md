# Data Types

Every type in Rust is of a certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data.

## Scalar Types

A *scalar* type represents a single value. Rust has four primary scalar types:

* integers
* floating-point numbers
* booleans
* characters

### Integer Types

An *integer* is a number without a fractional component.

Rust defaults integer types to `i32`.

**Integer Types**  

Length | Signed | Unsigned
-------|--------|---------
8-bit | `i8` | `u8`
16-bit | `i16` | `u16`
32-bit | `i32` | `u32`
64-bit | `i64` | `u64`
128-bit | `i128` | `u128`
arch | `isize` | `usize`

*Signed* numbers can be both positive and negative (prefixed with a `+` or `-` **sign**).

*Unsigned* numbers can only be positive (no **sign** prefix).

Each signed variant can store numbers from **<code>-(2<sup>n-1</sup>)</code>** to **<code>2<sup>n-1</sup> - 1</code>** inclusive, where *`n`* is the number of bits that variant uses.

An `i8` can store numbers from **<code>-(2<sup>7</sup>)</code>** to **<code>2<sup>7</sup> - 1</code>**, which equals **`-128`** to **`127`**.

Unsigned variants can store numbers from **`0`** to **<code>2<sup>n</sup> - 1</code>**.

A `u8` can store numbers from **`0`** to **<code>2<sup>8</sup> - 1</code>**, which equals **`0`** to **`255`**.

`isize` and `usize` depend on the architecture of the computer your program is running on, which is denoted in the table as *arch*: 64-bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.

You can write integer literals in any of the forms in the following table. Note that number literals that can be multiple numeric types allow a type suffix, such as `57u8`, to designate the type. Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`, which will have the same value as if you had specified `1000`.

Number literals | Example
----------------|--------
Decimal | `98_222`
Hex | `0xff`
Octal | `0o77`
Binary | `0b1111_0000`
Byte (`u8` only) | `b'A'`

### Floating-Point Types

Rust also ahs two primitive types for *floating-point numbers*, which are numbers with decimal points: `f32` and `f64`.

The default is `f64` because on modern CPUs, it's rougly the same speed as `f32` but is capable of more precision.

All floating-point types are signed.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 2.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single-precision float, and `f64` has double precision.

### Numeric Operations

Rust supports the basic mathematical operations you'd expect for all the number types: addition, subtraction, multiplication, division, and remainder.

Integer division truncates toward zero to the nearest integer.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;
}
```

### The Boolean Type

A boolean type in rust has two possible values: `true` and `false`. 

Booleans are only one byte in size.

The Boolean type in rust is specified using `bool`.

```rust
fn main() {
    let t = true;

    let f: bool = false;
}
```

### The Character Type

`char` is Rust's most primitive alphabetic type.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Not e that `char` literals are specified iwth single quotes, as opposed to string literals, which use double quotes.

Rust's `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust.

Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a "character" isn't really a concept in Unicode, so your human intuition for what a "character" is may not match up with what a `char` is in Rust.

## Compound Types

*Compound types* can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### The Tuple Type

A *tuple* is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

A tuple is created by writing a comma-separated list of values inside parenthesis. Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

The program first creates a tuple and binds it to the variable `tup`. It thten uses a pattern with `let` to take `tup` and turn it into three separate variables: `x`, `y`, and `z`. This is called *destructuring* because it breaks the single tuple into three parts. Finally, the program prints the value of `y`, which is `6.4`.

We can also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access:

```rust
fn main() {
    let x = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates the tuple `x` and then accesses each element of the tuple using their respective indices. As wiht most programming languages, the first index of a tuple is 0.

The tuple without any values has a special name, *unit*. The value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don't return any other value.

### The Array Type

An *array* is a collection of values that share the same type. 

Arrays in Rust have a fixed length.

The values in an array are written as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than the heap, or when you want to ensure you will always have a fixed number of elements.

An array isn't as flexible as the vector type, though. A *vector* is a similar collection type provided by the standard library that *is* allowed to grow or shrink in size. If you're unsure whether to use an array or a vector, chances are you should use a vector.

Arrays are more useful when you know the number of elements will not need to change.

```rust
let months = [
    "January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"
];
```

You can write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

You can initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets:

```rust
let a = [3; 5];
```

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

If you try to access an invalid index, for instance using `10`, you will see the following error:

```
thread 'main' panicked at src/main.rs:<line>:<char>:
index out of bounds: the len is 5 but the index is 10
```