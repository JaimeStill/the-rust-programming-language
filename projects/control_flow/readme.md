# Control Flow

The most common constructs that let you control the flow of execution in Rust code are `if` expressions and loops.

## `if` Expressions

An `if` exprsesion allows you to branch your code depending on conditions. You provide a condition and then state, "if this code is met, run this block of code. If the condition is not met, do not run this block of code."

```rust
fn if_condition(value: i32) {
    if value < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

The condition *must* be a `bool`. If the condition isn't a `bool`, an error is thrown.

### Handle Multiple Conditions with `else if`

```rust
fn is_divisible(x: i32) {
    if x % 4 == 0 {
        println!("{x} is divisible by 4");
    } else if x % 3 == 0 {
        println!("{x} is divisible by 3");
    } else if x % 2 == 0 {
        println!("{x} is divisible by 2");
    } else {
        println!("{x} is not divisible by 4, 3, or 2");
    }
}
```

Rust only executes the block for the first `true` condition, and once it finds one, it doesn't even check the rest. `is_divisible(12)` will only ever match the first condition, even though it is true for all of the specificed conditions (minus the final `else` statement).

Using too many `else if` expressions can clutter your code, so if you have more than one, you might want to refactor your code. See [`match`](https://doc.rust-lang.org/book/ch06-02-match.html).

### Using `if` in a `let` Statement

Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

When conditionally assigning a value, all arguments must be the same type.

## Repetition with Loops

Rust provides several *loops*, which will run through the code inside the loop body to the end and then start immediately back at the beginning.

Rust has three kinds of loops: `loop`, `while`, and `for`.

### Repeating Code with `loop`

The `loop` keywords tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```rust
use std::{thread, time};

fn run_loop() {
    let mut it: i32 = 0;
    let second = time::Duration::from_secs(1);

    /*
        Will only stop when you halt the
        program with ctrl + c
    */
    loop {
        it += 1;
        println!("{it}");
        thread::sleep(second);
    }
}
```

You can place the `break` keyword within the loop to tell the program when to stop executing the loop.

`continue` is used to tell the loop to skip over any remaining code in the current iteration, and loop back to the beginning immediately.

### Returning Values from Loops

One use for `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it:

```rust
fn run_counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

You can also `return` from inside a loop. While `break` only exits the current loop, `return` always exits the current function.

### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote:

```rust
fn labeled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}
```

The outer loop has the label `'counting_up`, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first `break`, which doesn't specify a label, will exit the inner loop only. The `break 'counting_up;` statement will exit the outer loop. The code prints:

```sh
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

### Conditional Loops with `while`

A program will often need to evaluate a condition within a loop. While the condition is `true`, the loop runs. When the condition ceases to be `true`, the program calls `break`, stopping the loop. It's possible to implement behavior like this using a combination of `loop`, `if`, `else`, and `break`. However, this pattern is so common that Rust has a built-in language construct for it, called a `while` loop.

```rust
fn run_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

The construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break`, and it's clearer. While a condition evaluates to `true`, the code runs; otherwise it exits the loop.

### Looping Through a Collection with `for`

You can also use the `while` construct to loop over the elements of a collection, such as an array:

```rust
fn run_iterative_while() {
    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;

    while index < a.len - 1 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}
```

All five array items appear in the terminal. Even though `index` will reach a value of `5` at some point, the loop stops executing before trying to fetch a sixth value from the array.

This approach is error prone; we could cause the program to panic if the index value or test condition is incorrect. If you cahnged the definitino of the `a` array to have four elements but forget to update the condition to `while index < 4`, the code would panic. It's also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection:

```rust
fn run_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}
```

Using the `for` loop, you wouldn't need to remember to change any other code if you changed the number of values in the array. The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust.

Even in situations where you want to run some code a certain number of times, as in the countdown example that used a `while` loop above, most Rust developers would use a `for` loop. The way to do that would be to use a `Range`, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

Here's what the countdwon would look like using a `for` loop and the `rev` method, used to reverse the range:

```rust
fn run_countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
```