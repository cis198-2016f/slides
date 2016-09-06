# Ownership & Lifetimes

### CIS 198 Lecture 2

---
## Ownership & Borrowing

- Explicit ownership is the biggest new feature that Rust brings to the table!
- Ownership is all&sup1; checked at compile time!
- Newcomers to Rust often find themselves "fighting with the borrow checker"
   trying to get their code to compile

&sup1;*mostly*

???

- The ownership model is the biggest thing that Rust brings to the table, its
  claim to fame.
- Ownership is something that's checked at compile time and has as little
  runtime cost as possible.
- So it's zero (or very little) runtime cost, but you pay for it with a longer
  compilation time and learning curve. Which is where the phrase "fighting with
  the borrow checker" comes from, when you have to work around the compiler's
  restrictions to figure out how to do what you want.

---
## Data

- Running programs need to store data in memory (RAM)
- Data can be allocated on the stack or on the heap
- Stack: local, temporary storage, usually small pieces of data
    - Data usually automatically freed when it goes out of scope (when the stack
      frame is popped)
- Heap: large, persistent storage
    - Objects, arrays, etc.
    - How is data managed?

---
## Manual Memory Management

- When you need to allocate memory:

```c
int *x = malloc(sizeof(int));
```

- When you're done with memory:

```c
free(x);
```

---
### Manual Memory Management

- Pros:
    - Yes, it's fast!
- Cons:
    - Extremely error prone:
        - Dangling pointers, double frees, memory leaks, ...
    - Tedious to write and maintain

---
## Garbage Collection

- Periodically look at all of the data on the heap and figure out what's still
  "alive".
    - i.e., variable in the program is still pointing to it.
- Free memory which is no longer being used.

---
### Garbage Collection

- Pros:
    - Totally automatic - no memory usage bugs
    - Easy to write
- Cons:
    - Possibly significant computational overhead:
        - Pause your program execution to do GC (slower execution)
        - Concurrent threading for parallel GC (more CPU usage)
    - Memory overhead.
    - Not well suited for embedded systems or older hardware
    - Causes strange bugs if things are garbage collected too agggressively or not enough

---
## How do we do better?

- Get rid of GC altogether _and_ avoid having the programmer manage memory
- Have the compiler do insert "manual" memory management on your behalf
- The compiler can figure out when data is used for the first and last times

---
### How do we do better?

- Pros:
    - Fast execution - no GC overhead
    - No (easy to avoid) memory usage bugs
- Cons:
    - Slower compilation times
    - Compiler is not always all-knowing and might need hints
    - Causes bugs if compiler makes a mistake

???

- Not always clear how to measure speed of a programming language
- TODO: does this remark go on this slide?

---
## Ownership

- A variable binding _takes ownership_ of its data.
    - A piece of data can only have one owner at a time.
- When a binding goes out of scope, the bound data is freed.
    - Heap-allocated data is de-allocated.
- Data _must be guaranteed_ to outlive its references.

```rust
fn foo() {
    // Creates a Vec object.
    // Gives ownership of the Vec object to v1.
    let mut v1 = vec![1, 2, 3];

    v1.pop();
    v1.push(4);

    // At the end of the scope, v1 goes out of scope.
    // v1 still owns the Vec object, so it can be cleaned up.
}
```

???

So here are the basics.
- When you introduce a variable binding, it takes ownership of its data. And a
  piece of data can only have one owner at a time.
- When a variable binding goes out of scope, nothing has access to the data
  anymore, so it can be released. Which means, if it's on the heap, it can be
  de-allocated.
- And data must be guaranteed to outlive its references. Or, all references are
  guaranteed to be valid.

---
## Move Semantics

```rust
let v1 = vec![1, 2, 3];

// Ownership of the Vec object moves to v2.
let v2 = v1;

println!("{}", v1[2]); // error: use of moved value `v1`
```

- `let v2 = v1;`
    - We don't want to copy the data, since that's expensive.
    - The data cannot have multiple owners.
    - Solution: move the Vec's ownership into `v2`, and declare `v1` invalid.
- `println!("{}", v1[2]);`
    - We know that `v1` is no longer a valid variable binding, so this is an error.
- Rust can reason about this at compile time, so it throws a compiler error.

???

Here's another example:
- Line 1: declare a vector v1.
- Line 2: let v2 = v1. Ownership of the vector is moved from v1 to v2.
  - we don't want to move or copy the data, that's expensive and causes other
    bugs
  - we already know the data can't have multiple owners
- Line 3: try to print v1.
  - but since the vector has been moved out of v1, it is no longer a valid
    variable binding
- all of this happens at compile time.

---
## Move Semantics

- Moving ownership is a compile-time semantic; it doesn't involve _copying_ data
  during execution.
- Moves are implicit (via assignments); no need to use something like C++'s
  `std::move`.
    - Rust provides functions like `std::mem::replace` for more
      advanced ownership management.

???

- Moving ownership is an impliict operation done at compile time. No data is
  moved or copied around when your program is being run.
- The movement of data is automatic, you don't need to call anything like
  std::move (as in C++).
- But you can do more fine-grained ownership or memory movement with a number of
  standrard library functions, like std::mem::replace.

---
## Ownership

- Ownership does not always have to be moved.
- What would happen if it did? Rust would get very tedious to write:

```rust
fn vector_length(v: Vec<i32>) -> Vec<i32> {
    // Do whatever here,
    // then return ownership of `v` back to the caller
}
```
- This doesn't scale at all.
    - The more variables you had to hand back, the longer your return type would be!
    - Imagine having to pass ownership around for 5+ variables at a time :(

???

- Ownership doesn't have to be moved.
- If it did, you would also have to return ownership at the end of every
  function, or have all of your variables constantly going out of scope.
- This gets absurd very quickly, imagine having to return all of your function
  arguments as return values just to make sure they don't go out of scope.

---
## Borrowing

- Obviously, this is not the case.
- Instead of transferring ownership, we can _borrow_ data.
- A variable's data can be borrowed by taking a reference to the variable;
  ownership doesn't change.
    - When a reference goes out of scope, the borrow is over.
    - The original variable retains ownership throughout.

```rust
let v = vec![1, 2, 3];

// v_ref is a reference to v.
let v_ref = &v;

// use v_ref to access the data in the vector v.
assert_eq!(v[1], v_ref[1]);
```

???

- Obviously, this is not the case in Rust, otherwise the language would be
  impossible to use.
- Instead, we can temporarily transfer ownership by borrowing data.
- The way that borrowing works is: you can take a reference to the original
  variable and use it to access the data.
- When a reference goes out of scope, the borrow is over.
- However, the original variable retains ownership during the borrow and
  afterwards.

---
## Borrowing

- Caveat: this adds restrictions to the original variable.
- Ownership cannot be transferred from a variable while references to it exist.
    - That would invalidate the reference.

```rust
let v = vec![1, 2, 3];

// v_ref is a reference to v.
let v_ref = &v;

// Moving ownership to v_new would invalidate v_ref.
// error: cannot move out of `v` because it is borrowed
let v_new = v;
```

???

- This adds a caveat: ownership cannot be ransferred *from* a variable that is
  currently being borrowed, because that would invalidate the reference.

---
## Borrowing

```rust
/// `length` only needs `vector` temporarily, so it is borrowed.
fn length(vec_ref: &Vec<i32>) -> usize {
    // vec_ref is auto-dereferenced when you call methods on it.
    vec_ref.len()
}

fn main() {
    let vector = vec![];
    length(&vector);
    println!("{:?}", vector); // this is fine
}
```
- The type of `length` changed: `vec_ref` is passed by reference, so it's now an `&Vec<i32>`.
- References, like bindings, are *immutable* by default.
- The borrow is over after the reference goes out of scope (at the end of `length`).

---
## Borrowing

```rust
/// `push` needs to modify `vector` so it is borrowed mutably.
fn push(vec_ref: &mut Vec<i32>, x: i32) {
    vec_ref.push(x);
}

fn main() {
    let mut vector: Vec<i32> = vec![];
    let vector_ref: &mut Vec<i32> = &mut vector;
    push(vector_ref, 4);
}
```

- Data can also be borrowed by _mutable_ reference: `&mut vec_ref`.
    - `vec_ref` is a reference to a mutable `Vec`.
    - The type is `&mut Vec<i32>`, not `&Vec<i32>`.
- Different from a reference-which-is-mutable.

---
## Borrowing

```rust
/// `push` needs to modify `vector` so it is borrowed mutably.
fn push2(vec_ref: &mut Vec<i32>, x: i32) {
    // error: cannot move out of borrowed content.
    let vector = *vec_ref;
    vector.push(x);
}

fn main() {
    let mut vector = vec![];
    push2(&mut vector, 4);
}
```
- Error! You can't explicitly dereference `vec_ref` because that
  would change the ownership of the data.

---
## Borrowing

- Rust will auto-dereference variables...
    - When making method calls on a reference.
    - When passing a reference as a function argument.

```rust
/// `length` only needs `vector` temporarily, so it is borrowed.
fn length(vec_ref: &&Vec<i32>) -> usize {
    // vec_ref is auto-dereferenced when you call methods on it.
    vec_ref.len()
}

fn main() {
    let vector = vec![];
    length(&&&&&&&&&&&&vector);
}
```

---
## Borrowing

- You will have to dereference variables...
    - When writing into them.
    - When using other operators (`+`).

```rust
let mut a = 5;
let ref_a = &mut a;
*ref_a = 4;
println!("{}", *ref_a + 4);
// ==> 8
```

???

- The dot operator dereferences variables, and other operators like + don't.

---
## `ref`

```rust
let mut vector = vec![0];

// Equivalent:
let x1 = &vector;
let ref x2 = vector;
assert_eq!(x1, x2);
```

- When binding a variable, `ref` will take a reference to the data instead of
  taking ownership.
    - Take a mutable reference with `ref mut`.

---
## `ref`

```rust
let mut vectors = (vec![0], vec![1]);
match vectors {
    (ref v1, ref mut v2) => {
        v1.len();
        v2.push(2);.
    }
}
```
- Use `ref` and `ref mut` when binding variables inside match statements.

---
## `Copy` Types

- Rust defines a trait&sup1; named `Copy` that signifies that a type should be
    copied anywhere it would otherwise be moved.
- Most primitive types are `Copy` (`i32`, `f64`, `char`, `bool`, etc.)
- Types that contain references cannot be `Copy` (e.g. `Vec`, `String`).

```rust
let x: i32 = 12;
let y = x; // `i32` is `Copy`, so it's not moved :D
println!("x still works: {}, and so does y: {}", x, y);
```

&sup1; Like a Java interface or Haskell typeclass

???

This is why we've been using Vectors as examples in this slide set.

---
## Borrowing Rules
##### _The Holy Grail of Rust_
Learn these rules, and they will serve you well.

- You can't keep borrowing something after it stops existing.
- One object may have many immutable references to it (`&T`).
- **OR** _exactly one_ mutable reference (`&mut T`) (not both).
- That's it!

![](img/holy-grail.jpg)

---
### Borrowing Prevents...

- Iterator invalidation due to mutating a collection you're iterating over.
- This pattern can be written in C, C++, Java, Python, Javascript...
    - But may result in, e.g, `ConcurrentModificationException` (at runtime!)

```rust
let mut vs = vec![1,2,3,4];
for v in &vs {
    vs.pop();
    // error: cannot borrow `vs` as mutable because
    // it is also borrowed as immutable
}
```

- `pop` needs to borrow `vs` as mutable to modify the data.
- But `vs` is being borrowed as immutable by the loop!

---
### Borrowing Prevents...

- Dangling pointers (use-after-free)
- Valid in C, C++...

```rust
let y: &i32;
{
    let x = 5;
    y = &x; // error: `x` does not live long enough
}
println!("{}", *y);
```

- The full error message:

```
error: `x` does not live long enough
note: reference must be valid for the block suffix following statement
    0 at 1:16
...but borrowed value is only valid for the block suffix
    following statement 0 at 4:18
```

- This eliminates a _huge_ number of memory safety bugs _at compile time_.

???

As a side note, this technique of creating a block to limit the scope of a
variable (in this case x) is pretty useful.

---
## Example: Vectors

- You can iterate over `Vec`s in three different ways:

```rust
let mut vs = vec![0,1,2,3,4,5,6];

// Borrow immutably
for v in &vs { // Can also write `for v in vs.iter()`
    println!("I'm borrowing {}.", v);
}

// Borrow mutably
for v in &mut vs { // Can also write `for v in vs.iter_mut()`
    *v = *v + 1;
    println!("I'm mutably borrowing {}.", v);
}

// Take ownership of the whole vector
for v in vs { // Can also write `for v in vs.into_iter()`
    println!("I now own {}! AHAHAHAHA!", v);
}

// `vs` is no longer valid
```

---
## Lifetimes

- There's one more piece to the ownership puzzle: Lifetimes.
- This is what's happening behind the scenes.

---
## Lifetimes

- Imagine this:
  1. I acquire a resource.
  2. I lend you a reference to my resource.
  3. I decide that I'm done with the resource, so I deallocate it.
  4. You still hold a reference to the resource, and decide to use it.
  5. You crash 😿.

- We've already said that Rust makes this scenario impossible, but glossed over
    how.
- We need to prove to the compiler that _step 3_ will never happen before _step 4_.

---
## Lifetimes

- Ordinarily, references have an implicit lifetime that we don't need to care
    about:
    ```rust
    fn foo(x: &i32) {
        // ...
    }
    ```
- However, we can explicitly provide one instead:
    ```rust
    fn bar<'a>(x: &'a i32) {
        // ...
    }
    ```

- `'a`, pronounced "tick-a" or "the lifetime *a*" is a *named* lifetime
  parameter.
    - `<'a>` declares generic parameters, including lifetime parameters.
    - The type `&'a i32` is a reference to an `i32` that lives at least as
      long as the lifetime `'a`.

???

## Stop here briefly to discuss

---
## Lifetimes

- The compiler is smart enough not to need `'a` above, but this isn't always the
  case.
- Scenarios that involve multiple references or returning references often
  require explicit lifetimes.

---
## Multiple Lifetime Parameters

```rust
fn borrow_x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str;
```

- In `borrow_x_or_y`, all input/output references all have the same lifetime.
    - `x` and `y` are borrowed (the reference is alive) as long as the returned
      reference exists.

```rust
fn borrow_p<'a, 'b>(p: &'a str, q: &'b str) -> &'a str;
```

- In `borrow_p`, the output reference has the same lifetime as `p`.
    - `q` has a separate lifetime with no constrained relationship to `p`.
    - `p` is borrowed as long as the returned reference exists.

---
## Lifetimes

- Okay, great, but what does this all mean?
    - If a reference `R` has a lifetime `'a`, it is _guaranteed_ that it will not
        outlive the owner of its underlying data (the value at `*R`)
    - If a reference `R` has a lifetime of `'a`, anything else with the lifetime
      `'a` is _guaranteed_ to live as long `R`.
- This will probably become more clear the more you use lifetimes yourself.

---
### Structured Data With Lifetimes

- Any struct or enum that contains a reference must have an explicit lifetime.
- Normal lifetime rules otherwise apply.

```rust
struct Foo<'a, 'b> {
  v: &'a Vec<i32>,
  s: &'b str,
}
```

---
### `impl` Blocks

- Implementing methods on the `Foo` struct requires lifetime annotations too!
- You can read this block as "the implementation using the lifetimes `'a` and
    `'b` for the struct `Foo` using the lifetimes `'a` and `'b`."

```rust
impl<'a, 'b> Foo<'a, 'b> {
  fn new(v: &'a Vec<i32>, s: &'b str) -> Foo<'a, 'b> {
    Foo {
      v: v,
      s: s,
    }
  }
}
```

---
### Structs

- Structs (and struct members) can have lifetime parameters.

```rust
struct Pizza(Vec<i32>);
struct PizzaSlice<'a> {
    pizza: &'a Pizza,  // <- references in structs must
    index: u32,        //    ALWAYS have explicit lifetimes
}

let p1 = Pizza(vec![1, 2, 3, 4]);
{
    let s1 = PizzaSlice { pizza: &p1, index: 2 }; // this is okay
}

let s2;
{
    let p2 = Pizza(vec![1, 2, 3, 4]);
    s2 = PizzaSlice { pizza: &p2, index: 2 };
    // no good - why?
}
```

???

## Live demo!

TODO

---
### Structs

- Lifetimes can be constrained to "outlive" others.
    - Same syntax as type constraint: `<'b: 'a>`.

```rust
struct Pizza(Vec<i32>);
struct PizzaSlice<'a> { pizza: &'a Pizza, index: u32 }
struct PizzaConsumer<'a, 'b: 'a> { // says "b outlives a"
    slice: PizzaSlice<'a>, // <- currently eating this one
    pizza: &'b Pizza,      // <- so we can get more pizza
}

fn get_another_slice(c: &mut PizzaConsumer, index: u32) {
    c.slice = PizzaSlice { pizza: c.pizza, index: index };
}

let p = Pizza(vec![1, 2, 3, 4]);
{
    let s = PizzaSlice { pizza: &p, index: 1 };
    let mut c = PizzaConsumer { slice: s, pizza: &p };
    get_another_slice(&mut c, 2);
}
```

---
### `'static`

- There is one special, reserved lifetime.
- `'static` means that a reference may be kept (and will be valid) for the
  lifetime of the entire program.
- i.e. the data referred to will never go out of scope.

---
### `static`

```rust
static PI: f32 = 3.1419;
```

- Static variables have `'static` lifetimes.
- Global variable with fixed memory address.

```rust
let life_of_pi: &'static f32 = &PI;
```

- String literals are references (with lifetime `'static`) to `static str`s.

```rust
let s1: &str = "Hello";
let s2: &'static str = "World";
```

---
### `static`

```rust
static mut counter: i32 = 0;
```

- You can create mutable static variables, but you can only mutate them inside
  `unsafe` blocks.
    - Rust forces you to declare when you're doing things that are...
      ~~morally questionable~~ potentially going to crash your program.

---
### `const`

```rust
const PI: f32 = 3.1419;
```

- Defines constants that live for the duration of the program.
- Must annotate the type!
- Constants "live" for the duration of the program.
    - Think of them as being inlined every time they're used.
    - No guarantee that multiple references to the same constant are the same.

---
## HW02: A Mediocre BST

- Due Wednesday, 2016-09-14, 11:59pm.
- Write a very simple BST.
- Ownership, structs, enums, methods.
- Follows "Learning Rust With Entirely Too Many Linked Lists"
