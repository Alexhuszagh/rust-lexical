//! Convert between extended-precision and native floats/integers.

// FROM INT

/// Import FloatType from integer.
///
/// This works because we call normalize before any operation, which
/// allows us to convert the integer representation to the float one.
macro_rules! from_int {
    ($int:ident) => ({
        FloatType {
            frac: $int as u64,
            exp: 0,
        }
    })
}

// FROM FLOAT

/// Import FloatType from native float.
///
/// Generate fraction from mantissa and read exponent as signed magnitude value.
macro_rules! from_float {
    ($float:ident, $exponent:ident, $hidden:ident,
     $fraction:ident, $bias:ident, $sig_size:ident)
    => ({
        let bits = $float.to_bits() as u64;
        let mut fp = FloatType {
            frac: (bits & $fraction),
            exp: ((bits & $exponent) >> $sig_size) as i32,
        };

        if fp.exp != 0 {
            fp.frac += $hidden;
            fp.exp -= $bias;
        } else {
            fp.exp = -$bias + 1;
        }

        fp
    })
}

// AS FLOAT

/// Export extended-precision float to native float.
///
/// The extended-precision float must be in native float representation,
/// with overflow/underflow appropriately handled.
macro_rules! as_float {
    ($self:ident, $float:tt, $int:ty, $denormal:ident, $hidden:ident,
     $fraction:ident, $bias:ident, $max:ident, $inf:ident, $sig_size:ident)
    => ({
        // Export floating-point number.
        if $self.frac == 0 || $self.exp < $denormal {
            // sub-denormal, underflow
            0.0
        } else if $self.exp >= $max {
            // overflow
            $float::from_bits($inf)
        } else {
            // calculate the exp and fraction bits, and return a float from bits.
            let exp: $int;
            if ($self.exp == $denormal) && ($self.frac & $hidden) == 0 {
                exp = 0;
            } else {
                exp = ($self.exp + $bias) as $int;
            }
            let exp = exp << $sig_size;
            let frac = $self.frac & $fraction;
            $float::from_bits(frac as $int | exp)
        }
    })
}
