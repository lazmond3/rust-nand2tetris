use crate::bit::Bit;
use crate::constant::BIT_WIDTH;
use crate::not::not;
use crate::word::{one_bit_word, InternalWord, Word};
use std::default::Default;

use std::convert::{TryFrom, TryInto};
use std::result::Result;

pub fn convert_vec_to_internal_word(a: Vec<Bit>) -> InternalWord {
    let boxed_slice = a.clone().into_boxed_slice();
    let boxed_array: Box<InternalWord> = match boxed_slice.try_into() {
        Ok(ba) => ba,
        Err(_) => {
            panic!(
                "Expected a Vec of length {} but it was {}.",
                BIT_WIDTH,
                a.len()
            )
        }
    };
    *boxed_array
}

//https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
// pub fn convert_vec_to_internal_word(a: Vec<Bit>) -> InternalWord {
/*
util.rs(32, 10): the trait
`std::convert::From<std::iter::Map<std::vec::IntoIter<bit::Bit>,
[closure@src/util.rs:30:14: 30:24]>>` is not implemented for `[bit::Bit; 16]`
*/
// a.into_iter()
//     .map(|x| not(x))
//     // .collect::<Vec<Bit>>()
//     .try_into()
//     .unwrap_or_else(|v: Vec<Bit>| {
//         panic!(
//             "Expected a Vec of length {} but it was {}.",
//             BIT_WIDTH,
//             v.len()
//         )
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_convert_vec_to_internal_word() {
        let word_00: Word = one_bit_word(0);
        let vec = word_00.internal().to_vec();
        let word_from_vec = Word::new(convert_vec_to_internal_word(vec));

        assert_eq!(word_00, word_from_vec);
        // for i in 0..BIT_WIDTH {
        //     assert_eq!(word_00, word_from_vec);
        // }
    }
}
