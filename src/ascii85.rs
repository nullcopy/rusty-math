use std::io::stdin;

enum Ascii85Mode {
    Encode,
    Decode,
}

pub fn ascii85_codec() {
    let (mode, input) = get_input();

    // Decode or encode input string
    let output = match mode {
        Ok(Ascii85Mode::Decode) => ascii85_decode(&input),
        Ok(Ascii85Mode::Encode) => ascii85_encode(&input),
        Err(e) => {
            println!("{}", e);
            String::from("")
        }
    };

    println!("{}", output);
    // Debug
    // match mode {
    //     Ok(Ascii85Mode::Decode) => println!("Decode: {:?}", input),
    //     Ok(Ascii85Mode::Encode) => println!("Encode: {:?}", input),
    //     Err(e) => println!("{}", e),
    // }
}

fn get_input() -> (Result<Ascii85Mode, String>, String) {
    println!(
        "Please enter 'd' or 'e' followed by a string\
        of text to decode or encode respectively:"
    );

    // Read input
    let mut s = String::new();
    stdin().read_line(&mut s).expect(
        "Please enter something to decode/encode from/to ascii85!",
    );

    // Detect mode
    let mode = match s.split_whitespace().count() {
        0...1 => Err("Error! Too few arguments provided.".to_string()),
        _ => {
            match s.split_whitespace().next() {
                Some("D") | Some("d") => Ok(Ascii85Mode::Decode),
                Some("E") | Some("e") => Ok(Ascii85Mode::Encode),
                _ => Err("Error! Unknown ascii85 mode specified.".to_string()),
            }
        }
    };

    // Remove encode/decode command (first two chars) from
    // the beginning of input string
    let _cmd: String = s.drain(..2).collect();

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return (mode, s);
}

fn ascii85_decode(s: &str) -> String {
    return String::from("");
}

fn ascii85_encode(s: &str) -> String {
    // Concatenate bytes into 32 bit numbers (groups of 4)
    let mut nums: Vec<u32> = Vec::new();
    for w in s.as_bytes().chunks(4) {
        // Concatenate bytes in groups of 4
        let concat = w.iter().fold(0, |acc: u32, &x| (acc << 8) + (x as u32));

        // Fix byte alignment if chunk was less than 4
        let concat = concat * (256 as u32).pow(4 - w.len() as u32);

        // Push new 32b number
        nums.push(concat);
    }

    // Divide 32b numbers into radix 85 numbers
    let mut rad85: Vec<u8> = Vec::new();
    for mut x in nums {
        println!("{}", x);
        let mut temp: Vec<u8> = Vec::new();
        temp.push((x % 85 + 33) as u8);
        for _ in 0..4 {
            x /= 85;
            temp.push((x % 85 + 33) as u8);
        }

        // Fix endian-ness
        temp.reverse();

        // Append to rad85
        rad85.append(&mut temp);
    }

    return String::from_utf8(rad85).unwrap_or("".to_string());
}
