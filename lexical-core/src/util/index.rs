//! Macro to facilitate indexing for unchecked variants.

/// Macro to index without bounds checking.
#[allow(unused_macros)]
macro_rules! unchecked_index {
    // Get
    ($container:ident[$index:expr]) => (
        * unsafe { $container.get_unchecked($index) }
    );

    // Get
    ($obj:ident$(.$subobj:ident)*[$index:expr]) => (
        * unsafe { $obj$(.$subobj)*.get_unchecked($index) }
    );
}

/// Macro to mutably index without bounds checking.
#[allow(unused_macros)]
macro_rules! unchecked_index_mut {
    // Get
    ($container:ident[$index:expr]) => {
        * unsafe { $container.get_unchecked_mut($index) }
    };

    // Set
    ($container:ident[$index:expr] = $rhs:expr) => (
        unsafe { *$container.get_unchecked_mut($index) = $rhs }
    );
}
