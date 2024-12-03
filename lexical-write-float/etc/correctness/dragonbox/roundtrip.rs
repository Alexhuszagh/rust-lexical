// Validate a round-trip for a given float.

use lexical_write_float::float::RawFloat;
use lexical_write_float::ToLexical;

#[inline(always)]
pub fn roundtrip<F>(float: F, buffer: &mut [u8]) -> Result<(), String>
where
    F: RawFloat + ToLexical + std::str::FromStr + std::string::ToString,
{
    let bytes = float.to_lexical(buffer);
    let string = unsafe { std::str::from_utf8_unchecked(bytes) };
    let roundtrip = string.parse::<F>().map_err(|_| float.to_string())?;
    let is_equal = if float.is_nan() {
        roundtrip.is_nan()
    } else {
        float == roundtrip
    };
    if !is_equal {
        return Err(float.to_string());
    }
    Ok(())
}
