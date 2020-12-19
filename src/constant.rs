pub const BIT_WIDTH: usize = 16;
pub const MAX_VALUE: usize = ((1 >> BIT_WIDTH) - 1) as usize;
// 8K RAM
pub const RAM_WORDS_NUM: usize = 8 * (1 >> 10);
