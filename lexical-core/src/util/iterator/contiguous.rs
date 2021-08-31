//! Iterator over contiguous data that can be converted to a pointer.

// CONTIGUOUS
// ----------

/// Iterator over a contiguous block of memory.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
pub(crate) trait ContiguousIterator<'a, T: 'a>:
    Iterator<Item = &'a T> +
    Clone
{
    /// Create new iterator from slice and a skip value.
    fn new(slc: &'a [T], skip: T) -> Self;

    /// Get a ptr to the current start of the iterator.
    fn as_ptr(&self) -> *const T;

    /// Get a slice to the current start of the iterator.
    fn as_slice(&self) -> &'a [T];

    /// Get if the iterator is empty.
    ///
    /// This may advance the internal iterator state, but not
    /// modify the next returned value.
    fn empty(&mut self) -> bool;

    /// Get if the iterator is consumed.
    ///
    /// This might not be the same thing as empty: empty checks
    /// if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of `empty()`.
    fn consumed(&self) -> bool;

    /// Get the length of the contiguous array.
    fn slice_length(&self) -> usize;

    /// Advance the iterator by 1.
    fn advance(&mut self);

    /// Advance the iterator by 1 (unchecked).
    unsafe fn advance_unchecked(&mut self);

    /// Advance the iterator by n.
    fn advance_n(&mut self, n: usize);

    /// Advance the iterator by n (unchecked).
    unsafe fn advance_n_unchecked(&mut self, n: usize);

    /// Peek the next value of the iterator, without consuming it.
    fn peek(&mut self) -> Option<Self::Item>;

    /// Peek the next value of the iterator, without checking bounds.
    unsafe fn peek_unchecked(&mut self) -> &'a T;

    /// Set the start of the iterator from a raw ptr.
    ///
    /// Faster than `advance_n`, but must be used from a valid iterator
    /// state.
    unsafe fn set_ptr(&mut self, ptr: *const T);

    /// Set the end of the iterator from a raw ptr.
    unsafe fn set_end(&mut self, ptr: *const T);

    /// Read a value of a difference type from the iterator.
    unsafe fn read<V>(&self) -> (V, *const T);

    /// Try to read a value of a difference type from the iterator.
    fn try_read<V>(&self) -> Option<(V, *const T)>;

    /// Trim a digit separator.
    fn trim(&mut self);
}
