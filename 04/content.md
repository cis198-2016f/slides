# Closures

### CIS 198 Lecture 4

???

So who's seen closures before?

---
## Closures
- A closure, anonymous function, or lambda function is a common paradigm in
  functional languages.
- In Rust, they're fairly robust, and match up well with the rest of Rust's
  ownership model.

```rust
let square = |x: i32| -> i32 { x * x };
println!("{}", square(3));
// => 6
```

???

Inline function definitions which can be bound to variables. The function block
is executed when the closure is called.

---
## Closure Syntax

```rust
fn foo(x: i32) -> i32 { x*x }

let foo_v1 = |x: i32| { x * x };
let foo_v3 = |x: i32| if x == 0 { 0 } else { 1 };
let foo_v4 = |x: i32, y: i32| {
    let z = x * y;
    x + y + z
};
```

- Syntactically pretty similar to function definitions.
- Specify arguments in `||`, followed by the return expression.
    - The return expression can be a series of expressions in `{}`.

???

- `let` instead of `fn`
- Arguments in pipes
- Braces are optional

---
## Type Inference

```rust
let square_v4 = |x: u32| { (x * x) as i32 };

let square_v4 = |x| -> i32 { x * x }; // ← unable to infer enough
let square_v4 = |x|        { x * x }; // ← type information!
```

- Unlike functions, we don't _need_ to specify the return type or argument types
  of a closure.
    - Unless the compiler can't infer the type of the argument(s) from
      the return expression.

???

- Having concrete function types for type inference and self-documentation.
- For closures, ease of use is more important.

---
## Closure Environment

- Closures _close_ over (contain) their environment.

```rust
let magic_num = 5;
let magic_johnson = 32;
let plus_magic = |x: i32| x + magic_num;
```

- The closure `plus_magic` is able to reference `magic_num` even though it's not
  passed as an argument.
    - `magic_num` is borrowed by the closure.
    - `magic_johnson` is not borrowed!

---
## Closure Environment

- If we try to mutably borrow `magic_num` later...

```rust
let mut magic_num = 5;
let magic_johnson = 32;
let plus_magic = |x: i32| x + magic_num;

let more_magic = &mut magic_num; // Err!
println!("{}", magic_johnson); // Ok!
```

```text
error: cannot borrow `magic_num` as mutable because it is also borrowed
as immutable [--explain E0502]

  |>
5 |>  let plus_magic = |x: i32| x + magic_num;
  |>                   --------     --------- previous borrow [..]
  |>                      |
  |>                      immutable borrow occurs here
6 |>
7 |>  let more_magic = &mut magic_num; // Err!
  |>                        ^^^^^^^^^ mutable borrow occurs here
9 |> }
```

???

Remember, you can't mutably borrow something that has existing references.

---
## Closure Environment

```rust
let mut magic_num = 5;
let magic_johnson = 32;
let plus_magic = |x: i32| x + magic_num;

let more_magic = &mut magic_num; // Err!
println!("{}", magic_johnson); // Ok!
```

- Why? `plus_magic` borrows `magic_num` when it closes over it!
- However, `magic_johnson` is not used in the closure, and its ownership is not
  affected.

---
## Closure Environment

- We can fix this kind of problem by making the closure go out of scope:

```rust
let mut magic_num = 5;
{
    let plus_magic = |x: i32| x + magic_num;
} // the borrow of magic_num ends here

let more_magic = &mut magic_num; // Ok!
println!("magic_num: {}", more_magic);
```

???

Questions?

---
## Move Closures

- As usual, closures are choose-your-own-ownership.
- Sometimes you don't want to take any references.
- You can force a closure to _take ownership_ of all environment
  variables by using the `move` keyword.
    - "Taking ownership" can mean creating a copy _or_ moving ownership.
    - _Disallows_ bringing references into the closure.

```rust
let mut magic_num = 5;
let own_the_magic = move |x: i32| x + magic_num;
let more_magic = &mut magic_num;
```

---
## Move Closures

- `move` closures are necessary when the closure `f` needs to outlive the scope in
  which it was created.
    - e.g. when you pass `f` into a thread, or return `f` from a function.

```rust
fn make_closure(x: i32) -> Box<Fn(i32) -> i32> {
    let f = move |y| x + y; // ^ more on this in like 15 seconds
    Box::new(f)
}

let f = make_closure(2);
println!("{}", f(3));
```

---
## Closure Ownership

- Sometimes, a closure _must_ take ownership of an environment variable to be
  valid. This happens automatically (without `move`):
- If a variable's ownership is moved inside the closure:
    ```rust
    let numbers = vec![2, 5, 32768];
    let alphabet_soup = || { numbers; vec!['a', 'b'] };
                          // ^ throw away unneeded ingredients
    println!("{:?}", numbers); // use of moved value
    ```
- e.g. into the return value.
    ```rust
    let lottery_numbers = vec![11, 39, 51, 57, 75];
    {
        let ticket = || { lottery_numbers };
    }
    // The braces do no good here.
    println!("{:?}", lottery_numbers); // use of moved value
    ```

- If the type is not `Copy`, the original variable is invalidated.

???

Rust actually gives you a warning about `numbers;` for being a "path statement
with no effect"

---
## Closure Ownership

- Closures which give away ownership can only be called once.
    - `move` behavior is implicit because `alphabet_soup` must own `numbers` to
      move it.

```rust
let numbers = vec![2, 5, 32768];
let alphabet_soup = || { numbers; vec!['a', 'b'] };
                      // ^ throw away unneeded ingredients
alphabet_soup();
alphabet_soup(); // use of moved value
```

```text
error: use of moved value: `alphabet_soup` [--explain E0382]
 --> <anon>:7:5
  |>
6 |>     alphabet_soup();
  |>     ------------- value moved here
7 |>     alphabet_soup(); // use of moved value
  |>     ^^^^^^^^^^^^^ value used here after move
note: move occurs because `alphabet_soup` has type
`[closure@<anon>:4:25: 4:55 numbers:std::vec::Vec<i32>]`,
which does not implement the `Copy` trait
```

---
## Closure Ownership

- Closures which own data but don't move it can be called multiple times.

```rust
let letters = vec!['a', 'b', 'π'];
let alphabet_soup = move || { println!("{:?}", letters) };
alphabet_soup();
alphabet_soup();
```

---
## Closure Ownership

- The same closure can take some values by reference and others by moving
  ownership (or Copying values).

```rust
let letters = vec!['a', 'b', 'π'];
let numbers = vec![2, 5, 32768];
let alphabet_soup = || { numbers; println!("{:?}", letters) };
// Owns `numbers` but borrows `letters`
```

---
## Closure Traits

- Closures are actually based on a set of traits under the hood!
    - `Fn`, `FnMut`, `FnOnce` - method calls are overloadable operators.

```rust
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call"
      fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call"
      fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call"
      fn call_once(self, args: Args) -> Self::Output;
}
```

---
## Closure Traits

- These traits all look pretty similar, but differ in the way they take `self`:
    - `Fn` borrows `self` as `&self`
    - `FnMut` borrows `self` mutably as `&mut self`
    - `FnOnce` takes ownership of `self`
- `Fn` is a superset of `FnMut`, which is a superset of `FnOnce`.
- Functions also implement these traits.

"The `|| {}` syntax for closures is sugar for these three traits. Rust will
generate a struct for the environment, impl the appropriate trait, and then use
it."&sup1;

&sup1;Taken from the Rust Book

???

Point out that FnOnce is the only one with an associated type; the other ones
inherit it.

---
## Closures As Arguments

- Passing closures works like function pointers.
- Let's take a (simplified) look at Rust's definition for `map`&sup1;.

```rust
// Self = Vec<A>
fn map<A, B, F>(self, f: F) -> Vec<B>
    where F: FnMut(A) -> B;
```

- `map` takes an argument `f: F`, where `F` is an `FnMut` trait object.
- You can pass regular functions in, since the traits line up!

&sup1;Real `map` coming in next lecture.

---
## Returning Closures

- You may find it necessary to return a closure from a function.
- Unfortunately, since closures are trait objects, they're unsized!

```rust
fn i_need_some_closure() -> (Fn(i32) -> i32) {
    let local = 2;
    |x| x * local
}
```

```
error: the trait `core::marker::Sized` is not implemented
    for the type `core::ops::Fn(i32) -> i32 + 'static`

note: `std::ops::Fn(i32) -> i32 + 'static` does not have a
constant size known at compile-time

note: the return type of a function must have a statically known size
```

- An `Fn` object is not of constant size at compile time.
    - The compiler doesn't know how to allocate space for the return value.

---
## Returning Closures

- Okay, we can fix this! Just wrap the `Fn` in a layer of indirection and return a reference!

```rust
fn i_need_some_closure_by_reference() -> &(Fn(i32) -> i32) {
    let local = 2;
    |x| x * local
}
```

```
error: missing lifetime specifier [--explain E0106]

help: this function's return type contains a borrowed value,
but there is no value for it to be borrowed from

help: consider giving it a 'static lifetime
```

- Now what? We haven't given this closure a lifetime specifier...
    - The reference we're returning must outlive this function.
    - But it can't, since that would create a dangling pointer.

---
## Returning Closures

- What's the right way to fix this? Use a `Box`!

```rust
fn box_me_up_that_closure() -> Box<Fn(i32) -> i32> {
    let local = 2;
    Box::new(|x| x * local)
}
```

```
error: closure may outlive the current function, but it
borrows `local`, which is owned by the current function [--explain E0373]
```

- So close!
- The closure we're returning is still holding on to its environment.
    - That's bad, since once `box_me_up_that_closure` returns, `local` will be destroyed.

---
## Returning Closures
- The good news? We already know how to fix this:

```rust
fn box_up_your_closure_and_move_out() -> Box<Fn(i32) -> i32> {
    let local = 2;
    Box::new(move |x| x * local)
}
```

???

The compiler actually gives you a hint about how to solve this now.
