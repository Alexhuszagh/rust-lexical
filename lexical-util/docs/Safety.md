# Safety

Due to the use of unsafe indexing, the guarantees as well the logic on how to safely implement the API is documented here.

The only major sources of unsafe code are wrapped in the [`iterator.rs`], [`skip.rs`], and [`noskip.rs`]. These are fully encapsulated into standalone traits to clearly define safety invariants and localize any unsafety to 1 or 2 lines of code.

The core, unsafe trait is [`DigitsIter`] and [`Iter`], both which expect to be backed by a contiguous block of memory (a slice) but may skip bytes internally. To guarantee safety, for non-skip iterators you must implement [`DigitsIter::is_consumed`][`is_consumed`] correctly.

This must correctly determine if there are any elements left in the iterator. If the buffer is contiguous, this can just be `index == self.len()`, but for a non-contiguous iterator it must skip any digits to advance to the element next to be returned or the iterator itself will be unsafe. **ALL** other safety invariants depend on this being implemented correctly.

To see if the cursor is at the end of the buffer, use [`is_buffer_empty`].

Any iterators must be peekable: you must be able to read and return the next value without advancing the iterator past that point. For iterators that skip bytes, this means advancing to the next element to be returned and returning that value.

For examples of how to safely implement skip iterators, you can do something like:

```rust
impl<_> DigitsIter<_> for MyIter {
    fn peek(&mut self) -> Option<u8> {
        loop {
            let value = self.bytes.get(self.index)?;
            if value != &b'.' {
                return value;
            }
            self.index += 1;
        }
    }
}
```

Then, [`next`] will be implemented in terms of [`peek`], incrementing the position in the cursor just after the value. The next iteration of peek will step to the correct byte to return.

```rust,ignore
impl<_> Iterator for MyIter {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.peek()?;
        self.index += 1;
        Some(value)
    }
}
```

[`is_buffer_empty`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#76
[`is_consumed`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#L276
[`peek`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#L284
[`next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
[`iterator.rs`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs
[`skip.rs`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/skip.rs
[`noskip.rs`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/noskip.rs
[`DigitsIter`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#L250
[`Iter`]: https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#L39
