//! High-level data interface utilities.

/// Convert format to interface, and call function with new item as first argument.
#[cfg(not(feature = "format"))]
macro_rules! apply_interface {
    ($fn:ident, $format:expr $(,$args:ident)*) => {
        $fn(StandardFastDataInterface::new($format) $(,$args)*)
    };
}

/// Convert format to interface, and call function with new item as first argument.
#[cfg(feature = "format")]
macro_rules! apply_interface {
    ($fn:ident, $format:expr $(,$args:ident)*) => {
        match $format & FloatFormat::FLAG_MASK {
            FloatFormat::PERMISSIVE         => $fn(PermissiveFastDataInterface::new($format) $(,$args)*),
            FloatFormat::STANDARD           => $fn(StandardFastDataInterface::new($format) $(,$args)*),
            FloatFormat::IGNORE             => panic!("..."),
            _                               => panic!("...")
        }
    };
}
