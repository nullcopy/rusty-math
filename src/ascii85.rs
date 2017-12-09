use std::io::stdin;

enum Ascii85Mode {
    Encode,
    Decode,
}

pub fn ascii85_codec() {
    let (mode, input) = get_input();

    // Decode or encode input string

    // Debug
    match mode {
        Ok(Ascii85Mode::Decode) => println!("Decode: {:?}", input),
        Ok(Ascii85Mode::Encode) => println!("Encode: {:?}", input),
        Err(e) => println!("{}", e),
    }
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
        },
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
