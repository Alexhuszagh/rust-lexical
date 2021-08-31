use lexical_parse_float::bigint::{Limb, StackVec};

pub fn vec_from_u32<const SIZE: usize>(x: &[u32]) -> StackVec<SIZE> {
    let mut vec = StackVec::<SIZE>::new();
    #[cfg(not(all(target_pointer_width = "64", not(target_arch = "sparc"))))]
    {
        for &xi in x {
            vec.try_push(xi as Limb).unwrap();
        }
    }

    #[cfg(all(target_pointer_width = "64", not(target_arch = "sparc")))]
    {
        for xi in x.chunks(2) {
            match xi.len() {
                1 => vec.try_push(xi[0] as Limb).unwrap(),
                2 => {
                    let xi0 = xi[0] as Limb;
                    let xi1 = xi[1] as Limb;
                    vec.try_push((xi1 << 32) | xi0).unwrap()
                },
                _ => unreachable!(),
            }
        }
    }

    vec
}
