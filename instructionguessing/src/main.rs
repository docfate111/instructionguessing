use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use u32;
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

fn bit_n(x: u32, n: u32) -> u32 {
    x >> n & 1
}

fn compare_bitfield(num: u32, start: u32, end: u32, actual: &str) -> bool {
    let mut expected = String::from("");
    for i in start..end + 1 {
        expected.push_str(&bit_n(num, i).to_string());
    }
    expected = expected.chars().rev().collect::<String>();
    return expected == actual;
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
                let mut isPossible = true;
                for word in &words[1..] {
                    if word.contains("..") {
                        let bit_range: Vec<&str> = word.split("..").collect();

                        start = match u32::from_str(bit_range[1]) {
                            Err(_) => {
                                return Err(format!("invalid int {}", bit_range[0]));
                            }
                            Ok(n) => Some(n),
                        };
                        end = match u32::from_str(bit_range[0]) {
                            Err(_) => {
                                return Err(format!("invalid int {}", bit_range[1]));
                            }
                            Ok(n) => Some(n),
                        };
                    } else if end.is_some()
                        && start.is_some()
                        && (word.contains("0") || word.contains("1"))
                    {
                        if word.len() == 1 {
                            if start.is_none() {
                                return Err(format!(
                                    "previous string is not an index for single bit"
                                ));
                            }
                            let value = word.parse::<u32>().unwrap();
                            if value != bit_n(num, start.unwrap()) {
                                isPossible = false;
                                break;
                            }
                        } else if word.len() as u32 != end.unwrap() - start.unwrap() + 1 {
                            return Err(format!(
                                "file {:?} invalid csv format
                                        {} {} {}",
                                path,
                                word,
                                start.unwrap_or(0),
                                end.unwrap_or(0)
                            ));
                        }
                        if !compare_bitfield(num, start.unwrap(), end.unwrap(), word) {
                            println!("start {} end {}", start.unwrap(), end.unwrap());
                            isPossible = false;
                            break;
                        }
                        // check num in the range
                        start = None;
                        end = None;
                    } else if word.len() <= 2 {
                        // case of single bit index
                        start = match u32::from_str(word) {
                            Err(e) => {
                                return Err(format!(
                                    "{:?} is a non-range number in the csv {}",
                                    word, e
                                ));
                            }
                            Ok(v) => Some(v),
                        };
                    } else {
                        return Err(format!("{:?} invalid csv format {}", path, word));
                    }
                }
                if isPossible {
                    println!("{} is a possilbe instruction", pos_instruction);
                } else {
                    println!("not possible");
                }
            }
            Ok(())
        }
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
