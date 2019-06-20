//! Pointer methods for Rustc versions before 1.26.0.

// POINTER METHODS

// Certain pointer methods aren't implemented below Rustc versions 1.26.
// We implement a dummy version here.

pub(crate) trait PointerMethods {
    // Add to the pointer (use padd to avoid conflict with ptr::add).
    unsafe fn padd(self, count: usize) -> Self;

    // Subtract from the pointer (use psub to avoid conflict with ptr::sub).
    unsafe fn psub(self, count: usize) -> Self;
}

impl<T> PointerMethods for *const T {
    #[inline(always)]
    unsafe fn padd(self, count: usize) -> Self {
        #[cfg(has_pointer_methods)]
        return self.add(count);

        #[cfg(not(has_pointer_methods))]
        return self.offset(count as isize);
    }

    #[inline(always)]
    unsafe fn psub(self, count: usize) -> Self {
        #[cfg(has_pointer_methods)]
        return self.sub(count);

        #[cfg(not(has_pointer_methods))]
        return self.offset((count as isize).wrapping_neg());
    }
}

impl<T> PointerMethods for *mut T {
    #[inline(always)]
    unsafe fn padd(self, count: usize) -> Self {
        #[cfg(has_pointer_methods)]
        return self.add(count);

        #[cfg(not(has_pointer_methods))]
        return self.offset(count as isize);
    }

    #[inline(always)]
    unsafe fn psub(self, count: usize) -> Self {
        #[cfg(has_pointer_methods)]
        return self.sub(count);

        #[cfg(not(has_pointer_methods))]
        return self.offset((count as isize).wrapping_neg());
    }
}
