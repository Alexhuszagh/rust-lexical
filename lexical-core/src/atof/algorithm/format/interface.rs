//! High-level data interface utilities.

/// Convert format to interface, and call function with new item as first argument.
#[cfg(not(feature = "format"))]
macro_rules! apply_interface {
    ($fn:ident, $options:expr $(,$args:ident)*) => {
        $fn(StandardFastDataInterface::new($options) $(,$args)*)
    };
}

/// Convert format to interface, and call function with new item as first argument.
#[cfg(feature = "format")]
macro_rules! apply_interface {
    ($fn:ident, $options:expr $(,$args:ident)*) => {
        // Parse Options.
        match $options.format().interface_flags() {
            NumberFormat::PERMISSIVE_INTERFACE  => $fn(PermissiveFastDataInterface::new($options) $(,$args)*),
            NumberFormat::STANDARD_INTERFACE    => $fn(StandardFastDataInterface::new($options) $(,$args)*),
            NumberFormat::IGNORE_INTERFACE      => $fn(IgnoreFastDataInterface::new($options) $(,$args)*),
            flags                               => {
                let integer = flags.intersects(NumberFormat::INTEGER_DIGIT_SEPARATOR_FLAG_MASK);
                let fraction = flags.intersects(NumberFormat::FRACTION_DIGIT_SEPARATOR_FLAG_MASK);
                let exponent = flags.intersects(NumberFormat::EXPONENT_DIGIT_SEPARATOR_FLAG_MASK);
                match (integer, fraction, exponent) {
                    (true, true, true)      => $fn(GenericIFEFastDataInterface::new($options) $(,$args)*),
                    (false, true, true)     => $fn(GenericFEFastDataInterface::new($options) $(,$args)*),
                    (true, false, true)     => $fn(GenericIEFastDataInterface::new($options) $(,$args)*),
                    (true, true, false)     => $fn(GenericIFFastDataInterface::new($options) $(,$args)*),
                    (false, false, true)    => $fn(GenericEFastDataInterface::new($options) $(,$args)*),
                    (false, true, false)    => $fn(GenericFFastDataInterface::new($options) $(,$args)*),
                    (true, false, false)    => $fn(GenericIFastDataInterface::new($options) $(,$args)*),
                    (false, false, false)   => $fn(GenericFastDataInterface::new($options) $(,$args)*)
                }
            }
        }
    };
}
