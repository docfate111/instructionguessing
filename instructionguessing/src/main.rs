use std::env;
use u32;
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
        Ok(i) => Ok(i),
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
