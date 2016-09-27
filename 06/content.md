# Pointers

### CIS 198 Lecture 6

###### Reference: [TRPL 5.8](https://doc.rust-lang.org/book/choosing-your-guarantees.html)

---
## `&T` and `&mut T`

- Your basic, economy-class references.
- Zero runtime cost; all checks are done at compile time.
- Not allowed to outlive their associated lifetime.
    - Can introduce serious lifetime complexity if you're not careful!
- Use these unless you _actually_ need something more complicated.

---
### Making References

- `Borrow`/`BorrowMut`: "a trait for borrowing data."&sup1;

```rust
trait Borrow<Borrowed> { fn borrow(&self) -> &Borrowed; }
```

- `AsRef`/`AsMut`: "a cheap, reference-to-reference conversion."&sup2;

```rust
trait AsRef<T>         { fn as_ref(&self) -> &T; }
```

- So... they're exactly the same?

&sup1; [Trait std::borrow::Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)

&sup2; [Trait std::convert::AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

---
### Making References

- No! While the have the same definition, `Borrow` carries additional connotations:
    - "If you are implementing Borrow and both Self and Borrowed implement Hash, Eq, and/or Ord, they must produce the same result."&sup1; &sup2;

&sup1; [Trait std::borrow::Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)

&sup2; [aturon on Borrow vs AsMut](https://github.com/rust-lang/rust/issues/24140#issuecomment-90626264)

---
## Borrow

```rust
pub trait Borrow<Borrowed> where Borrowed: ?Sized {
    fn borrow(&self) -> &Borrowed;
}
```

- Borrow lets you lend out references of other types, e.g.:
    - `Vec<T>: Borrow<Vec<T>>` lets you borrow a `Vec<T>` as a `&Vec<T>`
    - `Vec<T>: Borrow<[T]>` lets you borrow a `Vec<T>` as a `&[T]`
- This is how a `HashMap`'s `get` method is defined:

```rust
impl<K, V, S> HashMap<K, V, S> where K: Eq + Hash

fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V> 
    where K: Borrow<Q>, Q: Hash + Eq
```

- e.g. `String` implements `Borrow<str>`
```rust
// map: HashMap<String, i32>;
map.get("Foo"), Some(&42));
```

---
### Borrow

- Similar to `Deref`, which is the trait that lets you overload `*`.
- However, `Deref` uses an _associated type_. So a type `T` can have...
    - One implementation of `Deref { type Target; }`.
    - Many implentations of `Borrow<Borrowed>`.

???

TODO: in the future, this can be moved into the Traits lecture (?)

---
### AsRef

```rust
pub trait AsRef<T> where T: ?Sized {
    fn as_ref(&self) -> &T;
}
```

- Used when you want to explicitly convert between reference types.
- No other guarantees.
- e.g.
    - `impl AsRef<[u8]> for str`
    - `impl AsRef<str> for String`
    - `impl AsRef<[u8]> for String`
    - `impl<T> AsRef<T> for Box<T>`

---
### Making References

- Borrow has a generic implementation:
    - `impl<T> Borrow<T> for T`.
    - You can _always_ convert `T` to `&T`.
- `AsRef` actually has a different generic implementation:
    - `impl<'a, T, U> AsRef<U> for &'a T where T: AsRef<U>`
    - `impl<'a, T, U> AsRef<U> for &'a mut T where T: AsRef<U>`
    - That means `AsRef` has autodereferencing behavior.
    (`foo.as_ref()` = `(&foo).as_ref()` = `(&mut foo).as_ref()`)

---
## `From` One Type `Into` Another

- Casting (`as`) cannot be overloaded; instead, you can implement these traits:
- `trait From<T> { fn from(T) -> Self; }`
    - `Y::from(x)`
- `trait Into<T> { fn into(self) -> T; }`
    - `x.into()`
- Into comes with two generic implementations:
    - `From<T> for U` implies `Into<U> for T`.
        - So you should implement `From`.
    - into() is reflexive, i.e. `Into<T> for T` for all `T`.

---
## `Box<T>`

- `Box<T>` is one of Rust's ways of allocating data on the heap.
- A `Box<T>` is a pointer which owns a `T` on the heap.
- The allocated data is de-allocated when the pointer goes out of scope (via
  `Drop`).
- `Box<T>` implements `Deref<Target=T>`, can be dereferenced with `*`.
- Create a `Box` with `Box::new()`.

```rust
let boxed_five = Box::new(5);
```

---
## `Box<T>`

- Pros:
    - Easiest way to put something on the heap.
    - Zero-cost abstraction for dynamic allocation.
    - Shares typical borrowing and move semantics, including Deref.
    - Automatic destruction.
- Cons:
    - The `Box` binding is subject to general ownership rules.

---
## Box Ownership

- In homeworks, you've used have noticed patterns like this:

```rust
match self.head.take() {
    Some(boxed_node) => {
        // boxed_node: Box<Node>
        let unboxed_node = *boxed_node;
        let a = unboxed_node.elem;
        let b = unboxed_node.next;
    }
    None => (),
}
```

- But we said there's automatic dereferencing when you access struct elements?

---
## Box Ownership

```rust
match self.head.take() {
    Some(boxed_node) => {
        // boxed_node: Box<Node>
        let unboxed_node = *boxed_node;
        let a = unboxed_node.elem;
        let b = unboxed_node.next;
    }
    None => (),
}
```

- You can't _destructure_ Boxes (move only part of the struct) because that
  would leave the heap data only partially-valid.
- But you can move ownership to the local scope (by explicitly dereferencing).

---
## Box Syntax

- It's currently not possible to destructure the `Box` inside the `Option`. :(
- In Nightly Rust, it is, thanks to `box` syntax!

```rust
#![feature(box_syntax, box_patterns)]

let opt_box = Some(box 5);

match opt_box {
    Some(box value) => assert_eq!(value, 5),
    None => println!("None :("),
}
```

- The syntax might change, so it's behind a feature gate.

---
## Reference Counting

- Reference counting is a method of implementing garbage collection.
- Data is allocated on the heap.
- The data keeps track of how many pointers to it exists.
    - Increments when it's copied, decrements it goes out of scope.
    - When counter hits 0, data is de-allocated.
- Used by Python.

---
## `std::rc::Rc<T>`
- A "**R**eference **C**ounted" pointer.
- Call `clone()` on an `Rc` to get a reference.
    - Increments its reference count.
    - No data gets copied!
- When the ref count drops to 0, the value is freed.

---
## `std::rc::Rc<T>`

- The data can only be mutated when the reference count is 1.
    - `fn get_mut(this: &mut Rc<T>) -> Option<&mut T>`

```rust
use std::rc::Rc;

let mut shared = Rc::new(6);
{
    println!("{:?}", Rc::get_mut(&mut shared)); // ==> Some(6)
}
let mut cloned = shared.clone();
{
    println!("{:?}", Rc::get_mut(&mut shared)); // ==> None
    println!("{:?}", Rc::get_mut(&mut cloned)); // ==> None
}
```

---
## Weak Pointers

- Reference counting has weaknesses: if a cycle is ever created.
- Doubly-Linked List:
    - Node 1 has an `Rc` pointing to Node 2
    - Node 2 has an `Rc` pointing to Node 1
- `==>` Memory leak!
- This can be avoided with _weak references_.
    - Weak refs don't increment the _strong reference_ count, so they don't
      affect possible deallocation.
    - But that means they aren't always valid!

---
## `std::rc::Weak<T>`

- An `Rc` can be downgraded into a `Weak` pointer.
    - `fn downgrade() -> Weak<T>`.
- `Weak` pointers can only be upgraded back into `Rc`.
    - `fn upgrade() -> Option<Rc<T>>`

---
## Strong Pointers vs. Weak Pointers

- When do you use an `Rc` vs. a `Weak`?
    - Generally, you probably want to use `Rc`.
- If you want _possible access to data_ but no ownership, you might want to use
  a `Weak`.
    - Always take into account the possibility of upgrade failing.
- Any structure with reference cycles may also need `Weak`, to avoid the leak.
- Note: `Rc` cycles are actually difficult to create in Rust, because of mutability rules.

---
## `std::rc::Rc<T>`

- Pros:
    - Allows sharing ownership of data.
- Cons:
    - Has a (small) runtime cost.
        - Holds two reference counts (strong and weak).
        - Must update and check reference counts when cloning, upgrading, or
          dropping.
    - Reference cycles can potentially leak memory; must design data structure
      so it doesn't contain cycles.

???

So two more things about Rc.

---
### `std::sync::Arc<T>`

- An **A**tomic **R**eference-**C**ounted pointer
- Functionally the same as Rc, but uses thread-safe (atomic) reference counting.

---
## Rc Cycles

- Note: `Rc` cycles are actually difficult to create in Rust, because of mutability rules.

```rust
struct Node {
    next: Option<Rc<Node>>,
    prev: Option<Rc<Node>>,
}

let mut n1 = Rc::new(Node { next: None, prev: None });
let mut n2 = Node { next: None, prev: Some(n.clone()) };
// Can no longer get a mutable reference to n1
// because n2 has a pointer to it.
assert_eq(None, Rc::get_mut(&mut n)));
```

---
## Cells

- A way to wrap data to allow _interior mutability_.
- An _immutable_ reference allows modifying the contained value!
- There are two types of cell: `Cell<T>` and `RefCell<T>`.

```rust
struct Foo {
    x: Cell<i32>,
    y: RefCell<u32>,
}
```

---
## `std::cell::Cell<T>`

- A wrapper type providing interior mutability for `Copy` types.
    - `Cell<T>`s cannot contain references.
- Get _a copy_ of the value from a `Cell` with `get()`.
- _Replace_ the value inside a `Cell` with `set()`.
- Limited (only for making copies), but safe and cheap.

```rust
let c = Cell::new(10);
c.set(20);
println!("{}", c.get()); // 20
```

---
## `std::cell::Cell<T>`

- Pros:
    - Interior mutability.
    - No runtime cost!
    - Small allocation cost.
- Cons:
    - `Copy` types only.

---
## `std::cell::RefCell<T>`

- A wrapper type providing interior mutability for _any_ type.
- Uses _runtime_ borrow checking rules.
    - Same rules as compiler borrowchecker.
- Borrow inner data via `borrow()` or `borrow_mut()`.
    - Panics if the `RefCell` is already borrowed!

```rust
use std::cell::RefCell;

let refc = RefCell::new(vec![12]);
let mut inner = refc.borrow_mut();
inner.push(24);
println!("{:?}", *inner); // ==> [12, 24]

let inner2 = refc.borrow();
// ==> Panics since there is an existing mutable borrow
```

---
## `std::cell::RefCell<T>`

- Notice this _isn't_ thread-safe!
- There is no way (in stable Rust) to check if a borrow will panic before
  executing it.
    - `borrow_state(&self)` is an unstable way to do this.
    - Use a thread-safe Mutex instead.

---
## `std::cell::RefCell<T>`

- A common paradigm is putting a `RefCell` inside an `Rc` to allow shared
  mutability.
- Implement under-the-hood mutability (internal state changes).

---
## Clone Counters

```rust
use std::cell::Cell;

struct Rc<T> {
    ptr: *mut RcBox<T>
}

struct RcBox<T> {
    value: T,
    count: Cell<usize>
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        unsafe {
            (*self.ptr).count.set((*self.ptr).count.get() + 1);
            Rc { ptr: self.ptr }
        }
    }
}
```

---
## `std::cell::RefCell<T>`

- Pros:
    - Interior mutability for any type.
- Cons:
    - Stores an additional borrow state variable.
    - May panic at runtime.
    - Not thread-safe.

---
## `std::cell::Ref<T>` & `RefMut<T>`

- When you invoke `borrow()` on a `RefCell<T>`, you actually get `Ref<T>`, not `&T`.
    - Similarly, `borrow_mut()` gives you a `RefMut<T>`.
- These are pretty simple wrapper over `&T`, but define some extra methods.
    - Sadly, all of them are unstable pending the `cell_extras` feature 😞.

---
## `*const T` & `*mut T`

- C-like raw pointers: they just point... somewhere in memory.
- No ownership rules.
- No lifetime rules.
- Zero-cost abstraction... because there is no abstraction.
- Requires `unsafe` to be dereferenced.
    - May eat your laundry if you're not careful.
- Use these if you're building a low-level structure like `Vec<T>`, but not in
  typical code.
    - Can be useful for manually avoiding runtime costs.

---
## Raw Pointers

- Nothing unsafe about creating raw pointers:

```rust
let x = 5;
let raw_x: *const i32 = &x as *const i32;
```

- References can be implicitly coerced to raw pointers.
    - They have the same runtime representation!

```rust
fn foo(x: *const i32) { }

foo(&x);
foo(raw_x);
```

---
## Raw Pointers

- But you need an `unsafe` block to dereference them:

```rust
let x = 5;
let raw_x: *const i32 = &x as *const i32;

unsafe {
    println!("{}", *raw_x);
}
```

- Raw pointers must be explicitly cast to references:
    - By doing a dereference and reference operation (`&*`)

```rust
let x = 5;
let raw_x: *const i32 = &x;

unsafe {
    let ref_x = &*raw_x;
}
```

---
## Raw Pointers

- Can also create a raw pointer with `Box::into_raw`.
    - Consumes the box and returns the raw pointer.
    - Must remember to de-allocate the data.

```rust
let my_speed: Box<i32> = Box::new(88);
let my_speed: *mut i32 = Box::into_raw(my_speed);

// ...

unsafe {
    Box::from_raw(my_speed);
}
```

---
## Unsafe

- There are three things you can do in `unsafe`...
    - Update a `static mut` variable.
    - Dereference a raw pointer.
    - Call unsafe functions.
- That's it?

---
## Unsafe

- Lots of things are implemented as `unsafe` functions:
    - FFI
    - Compiler intrinsics
        - Memory operations: memcopy, memset, `mem::transmute`
        - `slice::get_unchecked`
        - Atomics, raw arithmetic operations
        - Compiler-internal flags such as getting type names, return address
          values.
    - Memory Operations
        - Allocate, deallocate memory.

???

Basically this + raw pointers = Vecs, Boxes, Rcs, etc.

---
## Unsafe

- There are four types of `unsafe` blocks...

```rust
unsafe fn foo() {
}

unsafe {
}

unsafe trait Spooky {
}

unsafe impl Spooky for i32 {
}
```

---
## Unsafe

- _Unsafe functions_ can only be called in _unsafe blocks_.
    - Functions which contain unsafe blocks can decide whether to expose a safe
      or unsafe API.
- _Unsafe traits can only be implemented in _unsafe impl blocks_.
