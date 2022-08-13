mod lib;

//importing required crates
use colored::*;
use lib::assembly_line;
use lib::lasagna;

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

    //Executing codes for assembbly line. Using assembly_line and colored mods
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
}