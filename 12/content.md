# Systems Programming

### CIS 198 Lecture 12

---
## So what _is_ systems programming?

---
## Options

- `rust-lang/libc`
- `nix-rust/nix`
- Higher-level stdlib functions

---
## libc

- `rust-lang` provided crate
- Direct, unsafe bindings to libc's C APIs
- Very very thin wrappers around system calls

---
## libc

- Use `libc` types `c_int` and `c_void` (pointers)

```rust
pub unsafe extern fn read(fd: c_int, buf: *mut c_void, count: size_t)
    -> ssize_t

unsafe {
    let mut buf: [u8; 10] = [0; 10];
    let buf_ref = buf.as_mut_ptr() as *mut c_void;
    let read_count = read(0, buf_ref, 10);
}
```

---
## nix

- Safe wrapper around libc functions for \*nix platforms (Linux, Darwin)
- Provides system calls using Rust idioms and types
- Internally wraps error handling (return values/errno) into `nix::Result<T>`
- Less complete support (but looking for contriutors)

---
## nix

```rust
// libc api (unsafe, manually handle return codes)
pub unsafe extern fn read(fd: c_int, buf: *mut c_void, count: size_t)
    -> ssize_t

// nix api (returns a nix::Result)
pub fn read(fd: RawFd, buf: &mut [u8]) -> Result<usize>
```

---
## stdlib

- The Rust stdlib provides a few modules, e.g.:
    - `env`: process arguments, environment variables
    - `mem` for (unsafe) memory manipulation
    - `process`: running other processes

---
## `#[no_std]`

- Produce a Rust binary that doesn't rely on the standard library.
    - Instead, you _have_ to link against `libc`
- You can disable the compiler-generated `main` function and use `#[start]` to
  designate your own.
```rust
// Entry point for this program
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
```
- Primarily used for embedded programming.

---
## `asm!`

- Directly embed assembly in your Rust programs!

---
## Let's Build A Shell!

Inspired by [kamalmarhubi/shell-workshop], a talk at Strange Loop 2014

[kamalmarhubi/shell-workshop]: https://github.com/kamalmarhubi/shell-workshop

---
### Starter Code

```bash
git clone https://github.com/cis198-2016f/shell-workshop.git
```

- Starts off with a working echo shell.

---
### Step 1: Execute Commands

- Step 1: read in a command and execute it.
- How do we execute commands? Well, we could use the syscall directly...

```rust
// libc api
pub unsafe extern fn execvp(c: *const c_char,
                            argv: *const *const c_char) -> c_int

// nix api
pub fn execvp(filename: &CString, args: &[CString]) -> Result<Void>
```

---
### Step 1: Execute Commands

- Or we can use this Rust stdlib wrapper...

```rust
Command::new("/usr/bin/ls")
    .arg("/bin/")
    .spawn() // use status() to have it respond with the exit code
    .expect("Failed to execute child");
```

- Hint: split a `String` into its tokens:

```rust
let tokens = input.split_whitespace().collect::<Vec<_>>();
```

???

Remember, we have to use this "turbofish" syntax to tell `collect` what type we
want it to return.

---
### Step 2: Forking

- Just kidding! `Command` doesn't take over your process like `execvp` does.
- Using `status()` will actually have it wait on the command to finish and
  report the error code.

---
### Step 2: cd

- Try to run `cd` in your shell right now.

---
### Step 2: cd

- `cd` is a shell _builtin_ command
    - It can't be run as a child process because that wouldn't effect the
      working directory of the shell process.
- Use `std::env` to change your working directory.

```rust
let new_dir = Path::new("/bin/");
env::set_current_dir(&new_dir).unwrap();
```

---
### Step 3: Pipelines

- `Command` provides `stdin()`, `stdout()` functions for configuring process IO
- Not yet actually part of this workshop :(
