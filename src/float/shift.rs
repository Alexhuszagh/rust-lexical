//! Macros for bit-wise shifts.

// SHIFT RIGHT

/// Shift extended-precision float right `shift` bytes.
macro_rules! shr {
    ($self:ident, $shift:expr) => ({
        $self.frac = $self.frac.wrapping_shr($shift as u32);
        $self.exp += $shift as i32;
    })
}

/// Shift extended-precision float left `shift` bytes.
macro_rules! shl {
    ($self:ident, $shift:expr) => ({
        $self.frac = $self.frac.wrapping_shl($shift as u32);
        $self.exp -= $shift as i32;
    })
}
