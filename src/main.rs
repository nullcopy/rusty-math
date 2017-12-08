extern crate clap;

use std::io::{stdin, stdout, Write};
use clap::{App, SubCommand};

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Does some basic math things.")
        .author("John B. <johnboydiv@gmail.com>")
        .subcommand(SubCommand::with_name("polydiv").about(
            "Perform polynomial division",
        ))
        .get_matches();

    // Determine which subcommand was chosen
    match cli_args.subcommand_name() {
        Some("polydiv") => polynomial_division(),
        _ => {
            println!("Error: No subcommand provided.");
            println!("Run 'rusty-math -h' for a list of available commands.");
        }
    }
}

fn polynomial_division() {
    // Read in numerator
    let n = get_input("Numerator coefficients: ").unwrap_or_else(|| {
        println!("Error: Please provide coefficients for the numerator polynomial!");
        std::process::exit(1);
    });

    // Parse the numerator into vectors of coefficients
    let n: Vec<f32> = n.split_whitespace()
        .map(|s| {
            s.parse().unwrap_or_else(|_e| {
                println!("Error: Coefficients must be integer values.");
                std::process::exit(1);
            })
        })
        .collect();

    // Read in denominator
    let d = get_input("Denominator: ").unwrap_or_else(|| {
        println!("Error: Please provide coefficients for the denominator polynomial!");
        std::process::exit(1);
    });

    // Parse the denominator into vectors of coefficients
    let d: Vec<f32> = d.split_whitespace()
        .map(|s| {
            s.parse().unwrap_or_else(|_e| {
                println!("Error: Coefficients must be integer values.");
                std::process::exit(1);
            })
        })
        .collect();

    // Print polynomial equation
    for i in 0..(n.len()-2) {
        print!("{}x^{} + ", n[i], (n.len() - i - 1));
    }
    print!("{}x + ", n[n.len()-2]);
    println!("{}", n[n.len()-1]);
    println!("--------");
    for i in 0..(d.len()-1) {
        print!("{}x^{} + ", d[i], (d.len() - i - 1));
    }
    print!("{}x + ", d[d.len()-2]);
    println!("{}", d[d.len()-1]);
}

fn get_input(msg: &str) -> Option<String> {
    // Input string
    let mut s = String::new();

    // Prompt for input
    print!("{}", msg);
    stdout().flush().unwrap();

    // Read input
    stdin().read_line(&mut s).expect(
        "Please enter a numerator!",
    );

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    if s.len() > 0 {
        return Some(s);
    } else {
        return None;
    }
}
