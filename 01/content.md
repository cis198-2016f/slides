# Structured Data

### CIS 198 Lecture 1

---
## Prologue: Modules

- Everything in Rust is module-scoped.
- If it's not marked `pub`, it's only accessible from within the same module.
- Modules can be defined with `mod`:

```rust
mod english {
    pub mod greetings {
    }
}

mod japanese {
    pub mod greetings {
    }
}
```

Reference: [TRPL 4.25](http://doc.rust-lang.org/book/crates-and-modules.html)

---
## Modules

```rust
mod english {
    pub mod greetings { /* ... */ }
}
```

- Modules can also be defined as separate files:
```
src/
├── lib.rs
└── english.rs
```
- `lib.rs`:
    ```rust
    mod english;
    ```
- `english.rs`:
    ```rust
    pub mod greetings { /* ... */ }
    ```

---
## Modules

- Modules can also be defined as directories:

```
src/
├── lib.rs
└── english
    ├── mod.rs
    └── greetings.rs
```

- `lib.rs`:
    ```rust
    mod english;
    ```
- `english/`
    - `mod.rs`:
        ```rust
        pub mod greetings;
        ```
    - `greetings.rs`:
        ```rust
        /* ... */
        ```

---
## Namespacing

- When accessing a member of a module, by default, namespaces
  are relative to the current module:

```rust
mod one {
    mod two { pub fn foo() {} }
    fn bar() {
        two::foo()
    }
}
```

- But it can be made absolute with a leading `::` operator:

```rust
mod one {
    mod two { pub fn foo() {} }
    fn bar() {
        ::one::two::foo()
    }
}
```

---
## `use`ing Modules

- `use` has the opposite rules.
- `use` directives are absolute by default:

```rust
use english::greetings;
```

- But can be relative to the current module:

```rust
// english/mod.rs
use self::greetings;
use super::japanese;
```

---
## Structured Data

- Rust has two simple ways of creating structured data types:
    - Structs: C-like structs to hold data.
    - Enums: OCaml-like; data that can be one of several types.

- Structs and enums may have one or more implementation blocks (`impl`s) which
  define methods for the data type.

---
## Structs

- A struct is a type with many fields.
- A struct declaration:
    - Fields are declared with `name: type`.

```rust
struct Point {
    x: i32,
    y: i32,
}
```

- By convention, structs have `CamelCase` names, and their fields have `snake_case` names.
- A struct instantiation:
    - Insert values instead of types.

```rust
let origin = Point { x: 0, y: 0 };
```

---
## Structs

- Struct fields are accessed with dot notation.
- Structs cannot be partially initialized.
    - You must assign all fields upon creation, or declare an uninitialized
      struct that you initialize later.

```rust
let mut p;
p = Point { x: 19, y: 8 };
p.x += 1;
p.y = p.x + 1;
```

---
## Structs

- Structs do not have field-level mutability.
- Mutability is a property of the **variable binding**, not the type.

```rust
struct Point {
    x: i32,
    mut y: i32, // error: expected identifier, found keyword `mut`
}
```

---
## Structs

- Structs are namespaced with their module name.
    - The fully qualified name of `Point` is `foo::Point`.
- Struct fields are private by default.
    - They are made public with the `pub` keyword.
- Private fields can only be accessed from within the module where the struct is
    declared.

```rust
mod foo {
    pub struct Point {
        pub x: i32,
        pub y: i32,
        len: i32,
    }
}

fn main() {
    let b = foo::Point { x: 12, y: 12, len: 0 };
    //      ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // error: field `len` of struct `foo::Point` is private
}
```

---
## Structs

```rust
mod foo {
    pub struct Point {
        x: i32,
        y: i32,
        len: i32,
    }

    // Creates and returns a new point
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y, len: y-x }
    }
}
```

- `new` is inside the same module as `Point`, so accessing private fields is
  allowed.
- Nothing special about the function `new` (except by convention).

---
### Struct `match`ing

- Destructure structs with `match` statements.

```rust
pub struct Point {
    x: i32,
    y: i32,
}

match p {
    Point { x, y } => println!("({}, {})", x, y)
}
```

???

- mentioned these in the first lecture, but this is where match is actually
  interesting

---
### Struct `match`ing

```rust
match p {
    Point { y, .. } => println!("{}", y)
}

match p {
    Point { y: y1, x: x1 } => println!("({}, {})", x1, y1)
}
```
- Fields do not need to be in order.
- List fields inside braces to bind struct members to those variable names.
    - Use `struct_field: new_var_binding` to change the variable it's bound to.
- Omit fields: use `..` to ignore all unnamed fields.

---
### Struct Update Syntax

- If you have an existing struct `x`, a struct initializer can use `.. x` to
  copy some or all struct fields from `x`.
- Any fields you don't specify in the initializer get copied over from the target struct.
- The struct used must be of the same type as the target struct.
    - No copying same-type fields from different-type structs!

```rust
struct Foo { a: i32, b: i32, c: i32 }

let mut x = Foo { a: 1, b: 1, c: 2 };
let x2 = Foo { a: 4, .. x };

// Useful to update multiple fields of the same struct:
x = Foo { a: 2, b: 2 .. x };
```

---
## Tuple Structs

- A struct variant with no named fields
- Numbered field accessors, like tuples (e.g. `x.0`, `x.1`, etc)
- Can also `match` these

```rust
struct Color(i32, i32, i32);

let mut c = Color(0, 255, 255);
c.0 = 255;
match c {
    Color(r, g, b) => println!("({}, {}, {})", r, g, b)
}
```

---
### Newtype Pattern

- Define a new type that's not just an alias.

```rust
// Cannot be compared or used interchangeably
struct Meters(i32);
struct Yards(i32);

// Simple type alias
// May be compared using `==`, added with `+`, etc.
type MetersAlias = i32;
type YardsAlias  = i32;
```

---
## Unit Structs

- A struct with no fields has zero size!
- We can still instantiate it.
- It can be used as a "marker" type on other data structures.
    - Useful to indicate, e.g., the type of data a container is storing.

```rust
struct Unit;

let u = Unit;
```

---
## Enums

- An enum, or "sum type", is a way to express some data that are one of several things.
- Much more powerful than enums in Java, C, C++, C#...
- Each enum variant can have:
    - no data (unit variant)
    - named data (struct variant)
    - unnamed ordered data (tuple variant)

```rust
enum Resultish {
    Ok,
    Warning { code: i32, message: String },
    Err(String)
}
```

---
## Enums

- Enum variants are namespaced by their enum type: `Resultish::Ok`.
    - You can import all variants with `use Resultish::*`.
- Enums, much as you'd expect, can be matched on like any other data type.

```rust
match make_request() {
    Resultish::Ok =>
        println!("Success!"),
    Resultish::Warning { code, message } =>
        println!("Warning: {}!", message),
    Resultish::Err(s) =>
        println!("Failed with error: {}", s),
}
```

---
## Recursive Types

- You might think to create a nice functional-style `List` type:

```rust
enum List {
    Nil,
    Cons(i32, List),
}
```

---
## Recursive Types

- ...unfortunately, this would have infinite size at compile time!
- Structs and enums are stored inline by default, so they cannot be recursive.
    - i.e. elements are not stored by reference, unless explicitly specified.
- The compiler tells us how to fix this, but what's a `box`?

```rust
enum List {
    Nil,
    Cons(i32, List),
}
// error: recursive type `main::List` has infinite size
// help: insert indirection (e.g., a `Box`, `Rc`, or `&`)
// at some point to make `List` representable
```

---
## Boxes

- A box is one of Rust's ways of allocating data on the heap.
- A `Box<T>` is a heap pointer with exactly one owner.
    - A `Box` owns its data (the `T`), which lives on the heap.
- Boxes are destructed when they go out of scope.
    - Freeing the data on the heap.
- Create a `Box` with `Box::new()`:

```rust
let boxed_five = Box::new(5);

enum List {
    Nil,
    Cons(i32, Box<List>), // OK!
}
```

---
## Patterns

- Use `...` to specify a range of values. Useful for numerics and `char`s.
- Use `_` to bind against any value (like any variable binding) and discard the
  binding.

```rust
let x = 17;

match x {
    0 ... 5 => println!("zero through five (inclusive)"),
    _ => println!("You still lose the game."),
}
```

---
### `match`: References

- Get a reference to a variable by asking for it with `ref`.

```rust
let x = 17;

match x {
    ref r => println!("Of type &i32: {}", r),
}
```

- And get a mutable reference with `ref mut`.
    - Only if the variable was declared `mut`.

```rust
let mut x = 17;

match x {
    ref r if x == 5 => println!("{}", r),
    ref mut r => *r = 5
}
```
- Similar to `let ref`.

---
### `if-let` Statements

- If you only need a single match arm, it often makes more sense to use Rust's `if-let` construct.
- For example, given the `Resultish` type we defined earlier:

```rust
enum Resultish {
    Ok,
    Warning { code: i32, message: String },
    Err(String),
}
```

---
### `if-let` Statements
- Suppose we want to report an error but do nothing on `Warning`s and `Ok`s.

```rust
match make_request() {
    Resultish::Err(_) => println!("Total and utter failure."),
    _ => println!("ok."),
}
```

- We can simplify this statement with an `if-let` binding:

```rust
let result = make_request();

if let Resultish::Err(s) = result {
    println!("Total and utter failure: {}", s);
} else {
    println!("ok.");
}
```

---
### `while-let` Statement

- `while-let` statements are very similar.
- Iterates until the condition fails to match.

```rust
while let Resultish::Err(s) = make_request() {
    println!("Total and utter failure: {}", s);
}
```

---
### Inner Bindings

- With more complicated data structures, use `@` to create variable bindings for
    inner elements.

```rust
#[derive(Debug)]
enum A { None, Some(B) }
#[derive(Debug)]
enum B { None, Some(i32) }

fn foo(x: A) {
    match x {
        a @ A::None              => println!("a is A::{:?}", a),
        ref a @ A::Some(B::None) => println!("a is A::{:?}", *a),
        A::Some(b @ B::Some(_))  => println!("b is B::{:?}", b),
    }
}

foo(A::None);             // ==> x is A::None
foo(A::Some(B::None));    // ==> a is A::Some(None)
foo(A::Some(B::Some(5))); // ==> b is B::Some(5)
```

---
## Associated Functions

```rust
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let p = Point::new(1, 2);
}
```

- Defined in an `impl` block.
- Individual functions must be made public with `pub`.
- Associated functions are called with namespacing syntax: `Point::new()`.
    - Not `Point.new()`.
- Like a static method in Java.

???

- Basically, it's something defined for the type and not for an _instance of_
  the type.
- Same for enums and structs.
- A constructor-like function is usually named `new`.
    - No inherent notion of constructors, no automatic construction.

---
## Implementations

- Methods, associated functions, and functions in general cannot be overloaded.
    - e.g. `Vec::new()` and `Vec::with_capacity(capacity: usize)` are both
      constructors for `Vec`
- Methods cannot be inherited.
    - Types must be composed instead.
    - However, traits (coming soon) have basic inheritance.

---
## Methods

```rust
impl Point {
    pub fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.distance();
}
```

- Also defined in an `impl` block.
- Methods are accessed via dot notation.
- Individual methods are made public with `pub`.
- The first argument is named `self`.

---
## Methods

- The type of `self` determines what kind of ownership the method uses.
- `&self`: the method *borrows* the value.
    - Use this unless you need a different ownership model.
- `&mut self`: the method *mutably borrows* the value.
    - The function needs to modify the struct it's called on.
- `self`: the method takes ownership.
    - The function *consumes* the value (and may or may not return a
      replacement).

---
## Methods

```rust
impl Point {
    // access but not modify fields
    fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    // modifies the struct
    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    // returns an entirely new struct and consumes the old one
    fn mirror_y(self) -> Point {
        Point { x: -self.x, y: self.y }
    }
}
```
