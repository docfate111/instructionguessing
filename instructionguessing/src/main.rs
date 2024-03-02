use std::env;
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
    println!("In file {}", file_path);
    Ok(())
}
