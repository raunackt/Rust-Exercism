mod lib;
mod rpg;

//importing required crates
use colored::*;
use lib::{assembly_line, lasagna, semi_structuted_logs};
use rpg::Player;

fn main() {
    //Execting functions for Lasagna. Using lasagna and colored mods
    println!("{}", "Lasagna".yellow());
    println!(
        "Expected time to make lasagna is {}",
        lasagna::expected_minutes_in_oven()
    );
    println!(
        "Remaing minutes are {}",
        lasagna::remaining_minutes_in_oven(20)
    );
    println!(
        "Time taken to prepare all layers is {}",
        lasagna::preparation_time_in_minutes(4)
    );

    println!(
        "Total elapse time is {}",
        lasagna::elapsed_time_in_minutes(4, 25)
    );
    println!();

    //Executing codes for assembly line. Using assembly_line and colored mods
    println!("{}", "Assembly Line".yellow());
    println!(
        "Prodcution rate at 4 speed is {}",
        assembly_line::production_rate_per_hour(4)
            .to_string()
            .green()
    );
    println!(
        "Working cars for 4 speed is {}",
        assembly_line::working_items_per_minute(4)
            .to_string()
            .green()
    );
    println!();

    //Semi structured logs
    println!("{}", "Log Level".yellow());
    let log_error = semi_structuted_logs::LogLevel::Error;
    println!(
        "{}",
        semi_structuted_logs::LogLevel::log(log_error, "There was an unexpected error").red()
    );
    println!(
        "{}",
        semi_structuted_logs::LogLevel::info("There have been changes to the file").blue()
    );
    let log_warning = semi_structuted_logs::LogLevel::Warning;
    println!(
        "{}",
        semi_structuted_logs::LogLevel::log(log_warning, "There are unused compotents")
            .bright_yellow()
    );
    println!();

    //Role Playing Game: GG
    println!("{}", "Role Playing Game".yellow());
    let mut new_player = Player {
        health: 23,
        mana: Some(45),
        level: 12,
    };

    println!("Damage caused: {}", new_player.cast_spell(3).to_string().green())
}
