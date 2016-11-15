# QuickCheck

### CIS 198 Lecture 13

- Lecture material inspired by/borrowed from Bill Labboon's [talk at Rust Belt Rust 2016][talk]

[talk]: https://github.com/laboon/RBR_QuickCheck

---
## Classic (Unit) Testing

- ...looks like this...

```rust
#[test]
fn test1() {
    let v = vec![5, 3, 2, 1];
    assert!(sort(v) == vec![1, 2, 4, 5];
}

#[test]
fn test2() {
    let v = vec![1, -4, 7];
    assert!(sort(v) == vec![-4, 1, 7];
}

#[test]
fn test3() {
    // ...
}

#[test]
fn test4() {
    // ...
}
```

---
## QuickCheck

- QuickCheck is a _property-based_ testing scheme.
- More popular among functional programming languages, but exists in some form
  for most languages
- Instead of writing input-output pairs, constrain the input properties and
  specify the output properties
- Classen, Hughes. "A Lightweight Tool for Random Testing of Haskell Programs".
  ICFP '00

---
## QuickCheck

```rust
fn sort(v: Vec<i32>) -> Vec<i32>;
```

- So what might the properties of a sorted array be?

---
## QuickCheck

1. Output array same size as passed-in array
2. Values in output array always increasing or staying the same
3. Value in output array never decreasing
4. Every element in input array is in output array
5. No element not in input array is in output array
6. Idempotent - running it again should not change output array
7. Pure - running it twice on same input array should always result in same output array

---
## Tests

- After you specify the set of properties, QuickCheck will generate test cases:

    - `[17, 19, 1] -> [1, 17, 19]` __OK__
    - `[-9, -100] -> [-100, -9]` __OK__
    - `[8, 2, 987, 287, 201] -> [2, 8, 201, 287, 987]` __OK__
    - `[101, 20, 32, -4] -> [-4, 20, 32, 101]` __OK__
    - `[115] -> [115]` __OK__
    - `[2, -9, -9, 1, 2] -> [-9, -9, 1, 2, 2]` __OK__
    - `[8, 3, 0, 4] -> [0, 3, 4, 8]` __OK__
    - `[17, 1009, -2, 413] -> [-2, 17, 413, 1009]` __OK__
    - `[12, 12, 1, 17, -100] -> [-100, 1, 12, 12, 17]` __OK__
    - `[] -> []` __OK__

---
## Failures

- What happens if invariants fail?
    - `[9, 0, -6, -5, 14] -> [0, -6, -5, 9, 14]` __FAIL__

- QuickCheck tries to _shrink_ it to the smallest failing test case.
    - `[9, 0, -6, -5, 14] -> [0, -6, -5, 9, 14]` __FAIL__
    - `[9, 0, -6] -> [0, -6, 9]` __FAIL__
    - `[-6, -5, 14] -> [-6, -5, 14]` __OK__
    - `[9, 0] -> [0, 9]` __OK__
    - `[0, -6] -> [0, -6]` __FAIL__
    - `[0] -> [0]` __OK__
    - `[-6] -> [-6]` __OK__

- This helps isolates exactly what's going wrong in your program.
    - `[0, -6] -> [0, -6]` __FAIL__

---
## Failures

- QuickCheck knows how to shrink on integers, floats, tuples, booleans, lists,
  strings, options and results.
    - Struct shrinking not supported (except via conversion-to-tuple)
- Not as useful for: IO-based testing
    - Writing to a file, socket
    - Displaying output (text, graphics)
- Great for:
    - Mathematical functions, pure functions
    - Well-specified problems

---
## Example

```rust
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

#[cfg(test)]
mod tests {
  use super::reverse;
  quickcheck! {
      fn prop_rev_twice(xs: Vec<u32>) -> bool {
          xs == reverse(&reverse(&xs))
      }
  }
}
```

- Taken from `BurntSushi/quickcheck`'s [README][qc]

[qc]: https://github.com/BurntSushi/quickcheck

---
## Example

- `quickcheck!` expands to something that looks like this:

```rust
#[test]
fn test_prop_rev_twice() {
    fn prop_rev_twice(xs: Vec<u32>) -> bool {
        xs == reverse(&reverse(&xs))
    }

    quickcheck(prop_rev_twice as fn(Vec<i32>) -> bool);
}
```

---
## Example

- On nightly Rust, where compiler plugins are available, use `#[quickcheck]`
  instead.

```rust
// Enable compiler plugins
#![feature(plugin)]
// Import plugins from quickcheck_macros crate
#![plugin(quickcheck_macros)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::reverse;
    #[quickcheck]
    fn prop_rev_twice(xs: Vec<u32>) -> bool {
        xs == reverse(&reverse(&xs))
    }
}
```

---
## Demo!

---
## TestResult

- By default, QuickCheck will run 100 input cases on each test.
- You can write a property test which only accepts certain types of inputs and
  discards others.
- Discarding is neither true nor false.

```rust
fn prop(xs: Vec<isize>) -> TestResult {
    if xs.len() != 1 {
        return TestResult::discard()
    }
    TestResult::from_bool(xs == reverse(&xs))
}
quickcheck(prop as fn(Vec<isize>) -> TestResult);
```

- By default, QuickCheck will try up to 10,000 inputs to find 100 inputs that
  your property accepts.

---
## Dependencies

```rust
[dependencies]
quickcheck = "0.3"
```

- If you're only using quickcheck in test code...
```rust
[dev-dependencies]
quickcheck = "0.3"
```

- If using `#[quickcheck]`
```rust
quickcheck_macros = "0.2"
```

---
## Exercise

- Write a function that takes a `Vec<i32>` and squares each element, then sorts
  the output.
- Write at least 4 quickcheck property tests.

---
## Stainless

- A lightweight, flexible, unopinionated testing framework.
- [`reem/stainless`][https://github.com/reem/stainless]

```rust
describe! stainless {
    before_each {
        // Start up a test.
        let mut stainless = true;
    }

    it "makes organizing tests easy" {
        // Do the test.
        assert!(stainless);
    }

    after_each {
        // End the test.
        stainless = false;
    }
}
```

???

Similar to RSpec syntax.

---
## Benchmarking

- Standard benchmarker provided by Rust's standard testing harness.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
```

```
test bench_add_two ... bench:       131 ns/iter (+/- 3)
```
