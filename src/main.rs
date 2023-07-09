/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///
/// 🦥 THX FOR LOOKING! <3 🦥 ❤
/// 
/// Warnings:
/// Please check every file before deleting it, i am not responsible for any damage caused by this tool!
/// 

static ASCII_LOGO_MSG: &str = r#"
 _       ________       __________________     
| |     / / ___/ |     / / ____/ ____/ __ \    
| | /| / /\__ \| | /| / / __/ / __/ / /_/ /    
| |/ |/ /___/ /| |/ |/ / /___/ /___/ ____/     
|__/|__//____/ |__/|__/_____/_____/_/         
"#;

mod cli;
mod colors;
mod scan;

use crate::colors::Colors;
use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    // Enable ansi support on windows
    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support()?;

    let args = match cli::parse_cli() {
        Ok(args) => {
            //clear the screen
            cls();

            args
        },
        Err(e) => {
            // Show the error before screen cleared and exits
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!("{}", ASCII_LOGO_MSG.bright_green());

    match scan::start_deletion(args.path, args.dry_run, args.verbose, args.age) {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    }

    same_line_input(&"Complete: Press any key to exit!".bright_green());

    Ok(())
}

pub fn same_line_input(msg: &str) -> String {
    print!("{}", msg);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


/// Clears the screen
fn cls() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}