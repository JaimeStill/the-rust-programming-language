# Ownership

## What Is Ownership?

*Ownership* is a set of rules that govern how a Rust program manages memory. Memory in Rust is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile.

### The Stack and The Heap

In a systems programming language like Rust, whether a value is on the stack or the heap affetcs how the language behaves and why you have to make certain decisions.

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.

* **stack** - stores values in the order it gets them and removes the values in the opposite order: *last in, first out*.
  * Adding data is called *pushing onto the stack*.
  * Removing data is called *popping off the stack*.
  * All data stored on the stack must have a known, fixed size.
  * Data with an unknown size at compile time or a size that might change must be stored on the **heap** instead.
  
* **heap** - when you put data on the heap, you request a certain amount of space. Thememory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of that location.
  * This process is called *allocating on the heap* and is sometimes abbreviated as just *allocating*.
  * Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store the new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform book keeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses. Once you understand ownership, you won't need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

### Ownership Rules

Keep these rules in mind:

* Each value in Rust has an *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

### Variable Scope

A scope is the range within a program for which an item is valid. Take the following variable:

```rust
let s = "hello";
```

The variable `s` refers to a string literal, where the value of the string is hardcoded into the text of the program. The variable is valid from the point at which it is delcared until the end of the current *scope*.

```rust
{
    // s is not value here, it's not yet declared

    // beyond this declaration, s is valid
    let s = "hello";

    // do stuff with s    
}

// the above scope is over, and s is no longer valid
```

There are two important points in time here:

* When `s` comes *into* scope, it is valid.
* It remains valid until it goes *out of scope*.

### The `String` Type

To illustrate the rules of ownership, a more complex data type is required than those covered to this point. The types covered previously:

* are of a known size
* can be stored on the stack, then popped off the stack when their scope is over
* can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.

We want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the `String` type is a great example.

We've already seen string literals, where a string value is hardcoded. String literals are convenient, but they aren't suitable for every situation in which we want to use text. One reason is that they're immutable. Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, `String`. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a `String` from a string literal using the `from` function:

```rust
let s = String::from("hello");
```

This kind of string *can* be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!");

println!("{s}");
```

### Memory and Allocation

In the case of a string literal, we knwo the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal's immutability.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

* The memory must be requested from the memory allocator at runtime.
* We need a way of returning this memory to the allocator when we're done with our string.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

The second part is different. In languages with a *garbage collector* (GC), the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it. In most languages without a GC, it's our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we'll waste memory. If we do it too early, we'll have an invalid variable. If we do it twice, that's a bug too. We need to pair exactly one `allocate` with exactly one `free`.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here's a version of the scope example using a `String` instead of a string literal:

```rust
{
    // s is not value here, it's not yet declared

    // beyond this declaration, s is valid
    let s = String::from("hello");

    // do stuff with s    
}

// the above scope is over, and s is no longer valid
```

There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called [**`drop`**](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and it's where the author of `String` can put the code to return the memory. Rust calls **`drop`** automatically at the closing curly bracket.

#### Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in Rust:

```rust
let x = 5;
let y = x;
```

This will "bind the value `5` to `x`; then make a copy of the value in `x` and bind it to `y`." We now have two variables, `x` and `y`, and both equal `5`. Integers are simple values with a known, fixed size, and these two `5` values are pushed onto the stack.

Now consider a `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar, so we might assume that the way it works would be the same, but that isn't quite what happens.

A `String` is made up of three parts:

1. A pointer to the memory that holds the contents of the string
2. A length
3. A capacity

This group of data is stored on the stack. The data referenced by the pointer is the memory on the heap that holds the contents.

![string-memory](https://doc.rust-lang.org/book/img/trpl04-01.svg)

* `len` is how much memory, in bytes, the contents of the `String` are currently using.

* `capacity` is the total amount of memory, in bytes, that the `String` has received from the allocator.

When we assign `s1` to `s2`, the `String` data from the stack is copied, meaning we copy: `ptr`, `len`, and `capacity`. We do not copy the data on the heap that `ptr` refers to:

![multiple-string-memory](https://doc.rust-lang.org/book/img/trpl04-02.svg)

Earlier, we said that when a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable. The above image shows both data pointers pointing to the same location. This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as a *double free* error and is a memory safety bug. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes out of scope.

The following will not compile:

```rust
let s1 = String::from("hello");
let s2 = s1;

/*
    error: borrow of moved value: `s1`
*/
println!("{s1}, world!");
```

Because Rust invalides the first variable, instead of being called a *shallow copy*, the copying of `ptr`, `len`, and `capacity` stack values without copying the heap data is referred to as a *move*. In this example, we would say that `s1` was *moved* into `s2`.

With only `s2` valid, when it goes out of scope it alone will free the memory. In addition, there's a design choice that's implied by this: Rust will never automatically create "deep" copies of your data. Therefore, any *automatic* copying can be assumed to be inexpensive in terms of runtime performance.

#### Variables and Data Interacting with Clone

If we *do* want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

This works just fine and explicitly produces:

![cloned-string](https://doc.rust-lang.org/book/img/trpl04-03.svg)

When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. It's a visual indicator that sometihng different is going on.

#### Stack-Only Data: Copy

```rust
let x = 5;
let y = x;
```

Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of teh actual values are quick to make. There's no reason we would want to prevent `x` from being valid after creating a variable `y`. `clone` wouldn't do anything different from what is happening here.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we'll get a compile-time error.

Here are some types that implement `Copy`:

* All integer types, such as `u32`.
* The boolean type, `bool`, with values `true` and `false`.
* All the floating-point types, such as `f64`.
* The character type, `char`.
* Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

### Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a functino will move or copy, just as assignment does:

```rust
fn fn_ownership() {
    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function
    takes_ownership(s);

    // s is no longer valid

    // x comes into scope
    let x = 5;

    /*
        x would move into the function,
        but i32 is Copy, so its okay
        to still use x afterward
    */
    makes_copy(x);
}
/*
    Here, x goes out of scope, then s. But because s's
    value was moved, nothing special happens.
*/

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
}
/*
    some_string goes out of scope and `drop` is called.
    The backing memory is freed.
*/

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}
// some_integer goes out of scope.
```

### Return Values and Scope

Returning values can also transfer ownership:

```rust
fn return_ownership() {
    // moves its return value into s1
    let s1 = gives_ownership();
    println!("s1: {s1}");

    // s2 comes into scope
    let s2 = String::from("hello");
    println!("s2: {s2}");

    let s3 = takes_and_gives_back(s2);
    println!("s3: {s3}");
}
/*
    s3 goes out of scope and is dropped.
    s2 was moved, so nothing happens.
    s1 goes out of scope and is dropped.
*/

/*
    moves its return value into
    the function that calls it
*/
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("yours");

    /*
        some_string is returned and moves
        out to the calling function.
    */
    some_string
}

// takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope.

    /*
        a_string is returned and moves
        out to the calling function
    */
    a_string
}

```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It's quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

Rust does let us return multiple values using a tuple:

```rust
/*
    this is a lot of work just to avoid
    s1 from going out of scope.
*/
fn tuple_return() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    // len() returns the length of the string
    let length = s.len();

    (s, length)
}
```

but this is too much ceremony and a lot of work for a concept that should be common. Luckily, Rust has a feature for using a value without transferring ownership, called *references*.

## References and Borrowing

## The Slice Type