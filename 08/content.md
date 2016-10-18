# I/O

### CIS 198 Lecture 8

---
## Traits!

```rust
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    // Other methods implemented in terms of read().
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    // Other methods implemented in terms of write() and flush().
}
```

- Standard IO traits implemented for a variety of types:
    - `File`s, `TcpStream`s, `Vec<T>`s, `&[u8]`s.
- Careful: return types are `std::io::Result`, not `std::Result`!
    - `type Result<T> = Result<T, std::io::Error>;`

---
## `std::io::Read`

```rust
use std::io;
use std::io::prelude::*;
use std::fs::File;

let mut f = try!(File::open("foo.txt"));
let mut buffer = [0; 10];

// read up to 10 bytes
try!(f.read(&mut buffer));
```

- `buffer` is an array, so the max length to read is encoded into the type.
- `read` returns the number of bytes read, or an `Err` specifying the problem.
    - A return value of `Ok(n)` guarantees that `n <= buf.len()`.
    - It can be `0`, if the reader is empty.
- Notice the imports: you need to import the struct `File` _and_ the trait
  `Read`.

---
## Ways of Reading

```rust
/// Required.
fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

/// Reads to end of the Read object.
fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>

/// Reads to end of the Read object into a String.
fn read_to_string(&mut self, buf: &mut String) -> Result<usize>

/// Reads exactly the length of the buffer, or throws an error.
fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>
```

- `Read` provides a few different ways to read into a variety of buffers.
    - Default implementations are provided for them using `read`.
- Notice the different type signatures.

---
## Reading Iterators

```rust
fn bytes(self) -> Bytes<Self> where Self: Sized

// Unstable!
fn chars(self) -> Bytes<Self> where Self: Sized
```

- `bytes` transforms some `Read` object into an iterator which yields
  byte-by-byte.
- The associated `Item` is `Result<u8>`.
    - So the type returned from calling `next()` on the iterator is
      `Option<Result<u8>>`.
    - Hitting an `EOF` corresponds to `None`.

- `chars` does the same, and will try to interpret the reader's contents as a
  UTF-8 character sequence.
    - Unstable; Rust team is not currently sure what the semantics of this
      should be. See issue [#27802][].

[#27802]: https://github.com/rust-lang/rust/issues/27802

---
## Iterator Adaptors

```rust
fn chain<R: Read>(self, next: R) -> Chain<Self, R>
    where Self: Sized
```
- `chain` takes a second reader as input, and returns an iterator over all bytes
  from `self`, then `next`.

```rust
fn take<R: Read>(self, limit: u64) -> Take<Self>
    where Self: Sized
```
- `take` creates an iterator which is limited to the first `limit` bytes of the
  reader.

---
## `std::io::Write`

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    // Other methods omitted.
}
```

- `Write` is a trait with two required methods, `write()` and `flush()`
    - Like `Read`, it provides other default methods implemented in terms of
      these.
- `write` (attempts to) write to the buffer and returns the number of bytes
  written (or queued).
- `flush` ensures that all written data has been pushed to the target.
    - Writes may be queued up, for optimization.
    - Returns `Err` if not all queued bytes can be written successfully.
---
## Writing

```rust
let mut buffer = try!(File::create("foo.txt"));

try!(buffer.write("Hello, Ferris!"));
```

---
## Writing Methods

```rust
/// Attempts to write entire buffer into self.
fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }

/// Writes a formatted string into self.
/// Don't call this directly, use `write!` instead.
fn write_fmt(&mut self, fmt: Arguments) -> Result<()> { ... }

/// Borrows self by mutable reference.
fn by_ref(&mut self) -> &mut Self where Self: Sized { ... }
```

---
## `write!`

- If you want string formatting, use the `write!` macro.
    - Between `write!` and `println!`, you never want to use string formatters
      directly.
- `write!` formatting as a wrapper around `write_fmt`.
- Returns a `Result`.

```rust
let mut buf = try!(File::create("foo.txt"));

write!(buf, "Hello {}!", "Ferris").unwrap();
```

---
## IO Buffering

- In general, IO operations are really slow.
- Why?

---
## IO Buffering

- Your running program has very few privileges.
- Reads and writes are done through the operating system.
    - Your program will do a _context switch_, pausing execution so
      the OS can gather input and relay it to your program.
    - This is veeeery slow.
- Doing a lot of reads in rapid succession suffers hugely if you make a system
  call on every operation.
    - Solve this with buffers!
    - Read a huge chunk at once, store it in a buffer, then access it
      as your program needs.

---
## BufReader

```rust
fn new(inner: R) -> BufReader<R>;
```
```rust
let mut f = try!(File::open("foo.txt"));
let buffered_reader = BufReader::new(f);
```

- `BufReader` is a struct that adds buffering to *any* reader.
- `BufReader` itself implements `Read`, so you can use it transparently.

---
## BufReader

- `BufReader` also implements a separate interface `BufRead`.

```rust
pub trait BufRead: Read {
    fn fill_buf(&mut self) -> Result<&[u8]>;
    fn consume(&mut self, amt: usize);

    // Other optional methods omitted.
}
```

---
## BufReader

- Because `BufReader` has access to a lot of data that has not technically been
  read by your program, it can do more interesting things.
- It defines two alternative methods of reading from your input, reading up
  until a certain byte has been reached.

```rust
fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>)
    -> Result<usize> { ... }
fn read_line(&mut self, buf: &mut String)
    -> Result<usize> { ... }
```

- It also defines two iterators.

```rust
fn split(self, byte: u8)
    -> Split<Self> where Self: Sized { ... }
fn lines(self)
    -> Lines<Self> where Self: Sized { ... }
```
---
## BufWriter

- `BufWriter` does the same thing, wrapping around writers.

```rust
let f = try!(File::create("foo.txt"));
let mut writer = BufWriter::new(f);
try!(buffer.write(b"Hello world"));
```

- `BufWriter` doesn't implement a second interface to control buffering.
- It just caches all writes until the it goes out of scope,
  then writes them all at once.

---
## `StdIn`

```rust
let mut buffer = String::new();

try!(io::stdin().read_line(&mut buffer));
```

- This is a very typical way of reading from standard input (terminal input).
- `io::stdin()` returns a value of `struct StdIn`.
- `stdin` implements `read_line` directly, instead of using `BufRead`.

---
## `StdInLock`

- A "lock" on standard input means only that current instance of `StdIn` can
  read from the terminal.
    - No two threads can read from standard input at the same time.
- All `read` methods call `self.lock()` internally.
- You can also create a `StdInLock` explicitly with the `stdin::lock()` method.

```rust
let lock: io::StdInLock = io::stdin().lock();
```

- A `StdInLock` instance implements `Read` and `BufRead`, so you can call any of
  the methods defined by those traits.

---
## `StdOut`

- Similar to `StdIn` but interfaces with standard output instead.
- Directly implements `Write`.
- You don't typically use `stdout` directly.
    - Prefer `print!` or `println!` instead, which provide string formatting.
- You can also explicitly `lock` standard out with `stdout::lock()`.

---
## Special IO Structs

- `repeat(byte: u8)`: A reader which will infinitely yield the specified byte.
    - It will always fill the provided buffer.
- `sink()`: "A writer which will move data into the void."
- `empty()`: A reader which will always return `Ok(0)`.
- `copy(reader: &mut R, writer: &mut W) -> Result<u64>`: copies all bytes from
  the reader into the writer.
