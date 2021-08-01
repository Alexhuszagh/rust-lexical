#[cfg(feature = "power-of-two")]
use lexical_util::format::NumberFormatBuilder;

#[cfg(feature = "power-of-two")]
pub const fn to_format(radix: u32) -> u128 {
    let builder = NumberFormatBuilder::new();
    let builder = builder.mantissa_radix(radix as u8);
    // 'e' is a valid digit for radix >= 15.
    if radix >= 15 {
        builder.exponent(b'^').build()
    } else {
        builder.build()
    }
}
