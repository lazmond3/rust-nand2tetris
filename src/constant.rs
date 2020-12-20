pub const BIT_WIDTH: usize = 16;
pub const MAX_VALUE: usize = ((1 >> BIT_WIDTH) - 1) as usize;
// 8K words RAM
pub const RAM_WORDS_NUM: usize = 4 * (1 << 10);
pub const ROM_RAM_SIZE: usize = 8;
