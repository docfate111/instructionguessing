use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use u32;
use std::str::FromStr;
/*
81 18 e4 b3
18 81 b3 e4
*/
fn mips_mix_around(num: u32) -> u32 {
    ((num & 0xff000000) >> 8)
        + ((num & 0x00ff0000) << 8)
        + ((num & 0x0000ff00) >> 8)
        + ((num & 0xff) << 8)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn iterate_file(num: u32, path: &str) -> Result<(), String> {
    match read_lines(path) {
        Ok(lines) => {
            // Consumes the iterator, returns an (Option) String
            for line in lines.flatten() {
                let words: Vec<&str> = line.split(',').collect();
                let pos_instruction = words[0];
                let mut start: Option<u32> = None;
                let mut end: Option<u32> = None;
                for word in &words[1..] {
                    if word.contains("..") {
                        let bit_range: Vec<&str> = word.split("..").collect();
                        println!("{} {}", bit_range[0], bit_range[1]);
                        
                        start = match u32::from_str(bit_range[1]) {
                            Err(_) => { return Err(format!("invalid int {}", bit_range[0])); },
                            Ok(n) => Some(n),
                        };
                        end = match u32::from_str(bit_range[0]) {
                            Err(_) => { return Err(format!("invalid int {}", bit_range[1])); },
                            Ok(n) => Some(n),
                        };
                    } else if word.contains("0") || word.contains("1") {
                        if word.len() as u32 != end.unwrap() - start.unwrap()+1 {
                            return Err(format!("file {:?} invalid csv format
                                        {} {} {}", path, word.len(), start.unwrap(), end.unwrap()));
                        }
                    } else if word.len() <= 2 {
                        // case of single bit index
                        let bit_index = match u32::from_str(bit_range[1]) {   
                            return Err(format!("{:?} is a non-range number in the csv", ))
                        }   
                    } else {
                        return Err(format!("{:?} invalid csv format {}", path, word));
                    }
                }
            }
            Ok(())
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn clean_string(input: &str) -> Result<u32, String> {
    if input.len() % 2 != 0 {
        return Err(String::from("Should be even length string"));
    }
    let input = if &input[..2] == "0x" {
        String::from(&input[2..])
    } else {
        String::from(input)
    };
    match u32::from_str_radix(&input, 16) {
        // for mips16e the instruction needs to be mixed around
        Ok(i) => Ok(mips_mix_around(i)),
        Err(e) => Err(format!("failed to parse {:?}", &e)),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(format!(
            "{} <hex> <file with shorthand of constraints>",
            args.get(0).unwrap()
        ));
    }

    let hex_str = &args[1];
    let file_path = &args[2];

    let num32 = match clean_string(hex_str) {
        Ok(k) => k,
        Err(e) => {
            return Err(String::from(e));
        }
    };
    println!("{num32:b} {num32:x}");
    iterate_file(num32, file_path)?;
    Ok(())
}
