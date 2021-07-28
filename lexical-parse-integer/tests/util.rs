pub const fn to_format(radix: u32) -> u128 {
    // TODO(ahuszagh) Need to actually implement...
    use lexical_util::format::MANTISSA_RADIX_SHIFT;
    (radix as u128) << MANTISSA_RADIX_SHIFT
}
