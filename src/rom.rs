use crate::constant::ROM_RAM_SIZE;
use crate::ram::Ram;
use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};
// use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;

    // load test
    #[test]
    fn for_rom_load() {
        let mut a = Rom::new();
        a.load("sample.txt")
    }

    // format test
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
}
