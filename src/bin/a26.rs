// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    let formatted_local_ru = local.format_localized("%A, %e %B %Y, %T", Locale::ru_RU).to_string();
    println!("Локальное время: {}", formatted_local_ru);   
}
