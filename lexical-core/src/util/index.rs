//! Macro to facilitate indexing for unchecked variants.

/// Macro to index without bounds checking.
#[cfg(feature = "unchecked_index")]
macro_rules! index {
    // Get
    ($container:ident[$index:expr]) => (
        * unsafe { $container.get_unchecked($index) }
    );
}

/// Macro to mutably index without bounds checking.
#[cfg(feature = "unchecked_index")]
macro_rules! index_mut {
    // Get
    ($container:ident[$index:expr]) => (
        * unsafe { $container.get_unchecked_mut($index) }
    );

    // Set
    ($container:ident[$index:expr] = $rhs:expr) => (
        unsafe { *$container.get_unchecked_mut($index) = $rhs }
    );
}

/// Macro to index with bounds checking.
#[cfg(not(feature = "unchecked_index"))]
macro_rules! index {
    // Get
    ($container:ident[$index:expr]) => (
        $container[$index]
    );
}

/// Macro to mutably index with bounds checking.
#[cfg(not(feature = "unchecked_index"))]
macro_rules! index_mut {
    // Get
    ($container:ident[$index:expr]) => (
        $container[$index]
    );

    // Set
    ($container:ident[$index:expr] = $rhs:expr) => (
        $container[$index] = $rhs
    );
}
