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
    let n = get_polynomial_coefficients("Numerator coefficients: ");

    // Read in denominator
    let d = get_polynomial_coefficients("Denominator coefficients: ");

    // Print polynomial equation
    print_polynomial(&n);
    println!("--------");
    print_polynomial(&d);
}

fn get_polynomial_coefficients(msg: &str) -> Vec<f32> {
    // Input string
    let mut s = String::new();

    // Prompt for input
    print!("{}", msg);
    stdout().flush().unwrap();

    // Read input
    stdin().read_line(&mut s).expect(
        "Please enter the coefficients of a polynomial!",
    );

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    if s.len() <= 0 {
        println!("Error: Please provide coefficients for the denominator polynomial!");
        std::process::exit(1);
    }

    // Parse the input string into a reverse ordered vector of coefficients,
    // with leading zeros removed, and return
    return s.split_whitespace()
        .rev()
        .map(|s| {
            s.parse().unwrap_or_else(|_e| {
                println!("Error: Coefficients must be integer values.");
                std::process::exit(1);
            })
        })
        .filter(|&x| x != 0.0)
        .collect();
}

fn print_polynomial(c: &Vec<f32>) {
    // Print polynomial from coefficients
    if c.len() > 2 {
        for i in (2..c.len()).rev() {
            print!("{}x^{} + ", c[i], i);
        }
    }
    if c.len() > 1 {
        print!("{}x + ", c[1]);
    }
    println!("{}", c[0]);
}
