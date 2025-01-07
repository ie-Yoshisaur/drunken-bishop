mod drunken_bishop;
mod drunken_crow;
mod parse_ssh_pubkey;

use crate::drunken_bishop::{generate_drunken_bishop_grid, render_drunken_bishop_art};
use crate::drunken_crow::{generate_drunken_crow_grid, render_drunken_crow_art};
use crate::parse_ssh_pubkey::parse_ssh_pubkey;
use std::env;

fn print_usage(program_name: &str) {
    eprintln!(
        "Usage: {} <public_key_file.pub> [--bishop | --crow]",
        program_name
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    if args.len() < 2 {
        print_usage(program_name);
        std::process::exit(1);
    }

    let pubkey_path = &args[1];
    let bin = parse_ssh_pubkey(pubkey_path)?;

    let mode = if args.len() >= 3 {
        match args[2].as_str() {
            "--crow" => "crow",
            "--bishop" => "bishop",
            other => {
                eprintln!("Unknown option: {}", other);
                print_usage(program_name);
                std::process::exit(1);
            }
        }
    } else {
        "bishop"
    };

    match mode {
        "crow" => {
            let (crow_grid, crow_start, crow_end) = generate_drunken_crow_grid(&bin);
            let crow_art = render_drunken_crow_art(&crow_grid, crow_start, crow_end);
            println!("==== Drunken Crow RandomArt ====\n{}", crow_art);
        }
        "bishop" => {
            let (bishop_grid, bishop_start, bishop_end) = generate_drunken_bishop_grid(&bin);
            let bishop_art = render_drunken_bishop_art(&bishop_grid, bishop_start, bishop_end);
            println!("==== Drunken Bishop RandomArt ====\n{}", bishop_art);
        }
        _ => {
            // This case should never be reached due to earlier checks
            eprintln!("Invalid mode selected.");
            std::process::exit(1);
        }
    }

    Ok(())
}
