lexical-size
============

Utilities to detect the binary size of lexical's numeric conversion routines. See [scripts/size.py](/scripts/size.py) for use.

We use I/O to ensure the routines are not optimized out: the empty baseline is the following:

```rust
use std::io::BufRead;

pub fn main() {
    println!("{}", std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .len()
    );
}
```

To avoid any differences in parsing or formatting binary sizes: everything is read as a string, and everything is written as a usize (using casts).
Since the parsers incur serious overhead for the writers, we read the bytes as a raw pointer which is significantly cheaper. For float formatters, the parsers can be the majority of the total binary size.

```rust
use std::io::BufRead;

pub fn main() {
    let value: f64 = unsafe {
        core::ptr::read_unaligned::<$t>(
            std::io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .trim()
                .as_bytes()
                .as_ptr() as *const _
        )
    };

    ...
}
```
