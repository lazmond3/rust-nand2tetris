use nand2tetris::bit::Bit;
use nand2tetris::word::Word;

fn main() {
    println!("Hello, world! bit I: {}", Bit::I);
    println!("{}", Word::bit_position(3).to_num());
    println!("num to bit3 to num: {}", Word::num_to_bit(3).to_num());
}
