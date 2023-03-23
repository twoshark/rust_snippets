Notes from https://stevedonovan.github.io/rust-gentle-intro

# Rust Array vs Slices

Arrays are fixed in size and not often used in Rust. This is because the type of the array includes its length, i.e. [f64; 4] or [i32; 2]. 

Slices are used instead with the notation: `&[i32]`. Slices are a 'view' of an array.

```
// array2.rs
// read as: slice of i32
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10,20,30,40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}
```

# `&` Operator

`&` is pronounced _borrow_ not _address of_ in rust. This just means pass by reference where ownership is maintained by the orginal owner.


# Rust does not have exception blocks i.e. try/catch syntax

- It manages `panic`s on a more line by line basis
- Ending a call with `?` means return an error if one is returned.
- `.expect("errMsg")` may be used to panic out a string error message when an error is return. 

# Vectors

The Rust type Vec (pronounced 'vector') behaves very much like an slice in fact; the difference is that you can append extra values to a vector
- note that it must be declared as mutable.

Vectors are expensive. When a vector is modified or created, it allocates from the heap and becomes the owner of that memory. The slice borrows the memory from the vector. When the vector dies or drops, it lets the memory go.

# Iterators

## `0..n`

This notation iterates from 0 through n and is used under the hood in `for` loops like: 

```rust
for var in iter {}
```

- Arrays maybe converted to iterators like so:
```rust
arr.iter()
```

- slices are implicitly converted to iterators and can be used directly in place

## Efficiency
It is more efficient to iterate over an array or slice this way than to use for i in 0..slice.len() {} because Rust does not have to obsessively check every index operation.

Vectors compare with each other and with slices by value.

# String types: `String` vs. `&str`

- `String` is a "allocated" string type

- `&str` pronounced _string slice_ is a "static" string type

- The borrow operator (&) can coerce a `String` into a `&str`
- Under the hood, String is basically a Vec<u8> and &str is &[u8], but those bytes must represent valid UTF-8 text.


# Matching

