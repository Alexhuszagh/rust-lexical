//! Macros to simplify working with format interfaces.

/// Branch based off of interface flags.
///
/// The branches use the following convention:
///     - iflag: internal digit separator flag
///     - lflag: leading digit separator flag
///     - tflag: trailing digit separator flag
///     - cflag: consecutive separator flag
///
/// From this, the remaining flags derived is just all combinations
/// of all flags of any length. These flags are then used in match
/// statements to identify the proper dispatcher.
#[macro_export]
#[cfg(feature = "format")]
macro_rules! generate_interface {
    (
        format => $format:ident,
        mask => $mask:ident,
        iflag => $iflag:ident,
        lflag => $lflag:ident,
        tflag => $tflag:ident,
        cflag => $cflag:ident,
        ifunc => $ifunc:expr,
        icfunc => $icfunc:expr,
        lfunc => $lfunc:expr,
        lcfunc => $lcfunc:expr,
        tfunc => $tfunc:expr,
        tcfunc => $tcfunc:expr,
        ilfunc => $ilfunc:expr,
        ilcfunc => $ilcfunc:expr,
        itfunc => $itfunc:expr,
        itcfunc => $itcfunc:expr,
        ltfunc => $ltfunc:expr,
        ltcfunc => $ltcfunc:expr,
        iltfunc => $iltfunc:expr,
        iltcfunc => $iltcfunc:expr,
        fallthrough => $fallthrough:expr,
        args => $($args:expr),* $(,)*
    ) => {{
        // Get our interface flags.
        const I: NumberFormat = NumberFormat::$iflag;
        const L: NumberFormat = NumberFormat::$lflag;
        const T: NumberFormat = NumberFormat::$tflag;
        const C: NumberFormat = NumberFormat::$cflag;
        const IL: NumberFormat = NumberFormat::from_bits_truncate(I.bits() | L.bits());
        const IT: NumberFormat = NumberFormat::from_bits_truncate(I.bits() | T.bits());
        const LT: NumberFormat = NumberFormat::from_bits_truncate(L.bits() | T.bits());
        const ILT: NumberFormat = NumberFormat::from_bits_truncate(IL.bits() | T.bits());
        const IC: NumberFormat = NumberFormat::from_bits_truncate(I.bits() | C.bits());
        const LC: NumberFormat = NumberFormat::from_bits_truncate(L.bits() | C.bits());
        const TC: NumberFormat = NumberFormat::from_bits_truncate(T.bits() | C.bits());
        const ILC: NumberFormat = NumberFormat::from_bits_truncate(IL.bits() | C.bits());
        const ITC: NumberFormat = NumberFormat::from_bits_truncate(IT.bits() | C.bits());
        const LTC: NumberFormat = NumberFormat::from_bits_truncate(LT.bits() | C.bits());
        const ILTC: NumberFormat = NumberFormat::from_bits_truncate(ILT.bits() | C.bits());

        match $format & NumberFormat::$mask {
            I       => $ifunc($($args,)*),
            IC      => $icfunc($($args,)*),
            L       => $lfunc($($args,)*),
            LC      => $lcfunc($($args,)*),
            T       => $tfunc($($args,)*),
            TC      => $tcfunc($($args,)*),
            IL      => $ilfunc($($args,)*),
            ILC     => $ilcfunc($($args,)*),
            IT      => $itfunc($($args,)*),
            ITC     => $itcfunc($($args,)*),
            LT      => $ltfunc($($args,)*),
            LTC     => $ltcfunc($($args,)*),
            ILT     => $iltfunc($($args,)*),
            ILTC    => $iltcfunc($($args,)*),
            _       => $fallthrough
        }
    }};
}
