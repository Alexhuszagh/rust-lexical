// TODO(ahuszagh) Should be moved to util

use crate::util::*;

pub(crate) trait Starts {
    fn with<'a, Iter1, Iter2>(iter1: Iter1, iter2: Iter2) -> (bool, Iter1)
    where
        Iter1: ContiguousIterator<'a, u8>,
        Iter2: ContiguousIterator<'a, u8>;
}

pub(crate) struct StartsWith {}

impl Starts for StartsWith {
    #[inline]
    fn with<'a, Iter1, Iter2>(mut l: Iter1, mut r: Iter2) -> (bool, Iter1)
    where
        Iter1: ContiguousIterator<'a, u8>,
        Iter2: ContiguousIterator<'a, u8>,
    {
        loop {
            // Only call `next()` on l if r is not None, otherwise,
            // we may incorrectly consume an l character.
            let ri = r.next();
            if ri.is_none() {
                return (true, l);
            } else if l.next() != ri {
                return (false, l);
            }
        }
    }
}

pub(crate) struct LowercaseStartsWith {}

impl Starts for LowercaseStartsWith {
    #[inline]
    fn with<'a, Iter1, Iter2>(mut l: Iter1, mut r: Iter2) -> (bool, Iter1)
    where
        Iter1: ContiguousIterator<'a, u8>,
        Iter2: ContiguousIterator<'a, u8>,
    {
        loop {
            let ri = r.next().map(|x| x.to_ascii_lowercase());
            if ri.is_none() {
                return (true, l);
            } else if l.next().map(|x| x.to_ascii_lowercase()) != ri {
                return (false, l);
            }
        }
    }
}
