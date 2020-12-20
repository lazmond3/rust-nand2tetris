use crate::constant::ROM_RAM_SIZE;
use crate::ram::Ram;
use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

struct Rom {
    rams: [Ram; ROM_RAM_SIZE],
}

impl Rom {
    pub fn new() -> Self {
        let ram_internal = [Ram::new(); ROM_RAM_SIZE];
        Rom { rams: ram_internal }
    }
    pub fn load(&mut self, file_name: &str) {
        // https://qiita.com/garkimasera/items/f39d2900f20c90d13259
        // expectは、値がSomeのときは　値を取り出し、そうではないときはpanic する。

        let file = File::open(file_name.clone()).expect(&format!("Fail to open {}", file_name));
        let mut reader = BufReader::new(file);
        for line in reader.lines() {
            println!("line: {} ", line.unwrap());
        }
    }
}

fn readme(file_name: &str) {
    let file = File::open(file_name).expect(&format!("failed open file: {}", file_name));
    let mut reader = BufReader::new(file);

    let mut buf = &mut [b'\0'; 100];
    reader.read(buf);
    let vec = buf.to_vec();

    let st = std::string::String::from_utf8(vec).expect(&format!("failed to from utf8"));

    println!("in readme: readed string: {}", st);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_rom_load() {
        let mut a = Rom::new();
        a.load("sample.txt")
    }
    #[test]
    fn format_test() {
        let file_name = "sample.txt";
        let v: std::string::String = format!("Fail to open {}", file_name);
        println!("print_test v: &str: {}", &v);
        println!("print_test v:  str: {}", v);
        let my_string = std::string::String::from("hello string &str");
        println!("print_test v:  my_string string: {}", my_string);
        println!("print_test v:  my_string &String is &str?: {}", &my_string)
    }

    #[test]
    fn read_test() {
        let lines = read_lines("sample.txt").expect(&format!("failed to give lines."));
        for line in lines {
            let u = line.unwrap();
            println!("line : {}", u);
        }
    }
}
