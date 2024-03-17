use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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

fn bit_n(x: u32, n: u8) -> u8 {
    (x >> n & 1).try_into().unwrap()
}

fn compare_bitfield(num: u32, start: u8, end: u8, actual: &str) -> bool {
    let mut expected = String::from("");
    for i in start..end + 1 {
        expected.push_str(&bit_n(num, i).to_string());
    }
    expected = expected.chars().rev().collect::<String>();
    expected == actual
}

fn get_bitfield(num: u32, start: u8, end: u8) -> String {
    let mut expected = String::from("");
    for i in start..end + 1 {
        expected.push_str(&bit_n(num, i).to_string());
    }
    expected
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
                let mut pos_instruction = match words.get(0) {
                    None => {
                        return Err("empty line or error parsing csv".to_string());
                    }
                    Some(v) => String::from(*v),
                };
                let mut start: Option<u8> = None;
                let mut end: Option<u8> = None;
                let mut is_possible = true;
                // check one possible instruction "words" with this loop
                for word in &words[1..] {
                    //println!("word {}", word);
                    if start.is_none() {
                        if word.contains("..") && word.matches('.').count() == 2 {
                            let bit_range: Vec<&str> = word.split("..").collect();

                            start = match u32::from_str(bit_range[1]) {
                                Err(_) => {
                                    return Err(format!("{}: invalid int {}", word, bit_range[0]));
                                }
                                Ok(n) => Some(n as u8),
                            };
                            end = match u32::from_str(bit_range[0]) {
                                Err(_) => {
                                    return Err(format!("{}: invalid int {}", word, bit_range[1]));
                                }
                                Ok(n) => Some(n as u8),
                            };
                        } else {
                            start = match u32::from_str(word) {
                                Err(_) => {
                                    return Err(format!("{} is not a valid bit index", word));
                                }
                                Ok(n) => Some(n as u8),
                            };
                        }
                    } else if start.is_some() && (word.contains('0') || word.contains('1')) {
                        if word.len() == 1 {
                            if start.is_none() {
                                return Err(
                                    "previous string is not an index for single bit".to_string()
                                );
                            }
                            let value = word.parse::<u32>().unwrap();
                            if value != bit_n(num, start.unwrap()).into() {
                                is_possible = false;
                                break;
                            }
                            pos_instruction.push_str(&format!(" [{}]:{}", start.unwrap(), word));
                            start = None;
                        } else if start.is_some() && end.is_some() {
                            if word.len() as u8 != end.unwrap() - start.unwrap() + 1 {
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
                                is_possible = false;
                                break;
                            }
                            pos_instruction.push_str(&format!(
                                " [{},{}]:{}",
                                end.unwrap(),
                                start.unwrap(),
                                word
                            ));

                            start = None;
                            end = None;
                        }
                    } else if word.contains('*') {
                        if start.is_some() && end.is_some() {
                            // if there were no bytes matching the range then
                            // store what bytes fit into what ranges
                            // i.e. 25...21 print the 25 to 21st byte from the input argument
                            pos_instruction.push_str(&format!(
                                " [{},{}]:{}",
                                end.unwrap(),
                                start.unwrap(),
                                &get_bitfield(num, start.unwrap(), end.unwrap())
                            ));
                        } else if start.is_some() {
                            pos_instruction.push_str(&format!(
                                " [{}]:{}",
                                start.unwrap(),
                                bit_n(num, start.unwrap())
                            ));
                        }
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
                            Ok(v) => Some(v as u8),
                        };
                    } else {
                        return Err(format!(
                            "{:?} invalid csv format {}\ninstruction so far {}",
                            path, word, pos_instruction
                        ));
                    }
                }
                if is_possible {
                    println!("{}", pos_instruction);
                }
                /*else {
                    println!("{} not possible", pos_instruction);
                }*/
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

    let num32 = clean_string(hex_str)?;
    iterate_file(num32, file_path)?;
    Ok(())
}
