use nand2tetris::bit::Bit;
use nand2tetris::word::Word;

fn main() {
    println!("Hello, world! bit I: {}", Bit::I);
    println!("{}", Word::bit_position(3).to_num());
}
