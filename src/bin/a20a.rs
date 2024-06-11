use std::io;

enum PowerState {
    On,
    Off,
    Sleep
}

fn input_to_enum(input: &str) -> Option<PowerState> {
    let input_lower = input.to_lowercase();
    match input_lower.as_str() {
        "on" => Some(PowerState::On),
        "off" => Some(PowerState::Off), 
        "sleep" => Some(PowerState::Sleep),
        &_ => todo!()
    }
}

fn print_state(state: PowerState) {
    match state {
        PowerState::On => println!("Включено"),
        PowerState::Off => println!("Выключено"),
        PowerState::Sleep => println!("Sleep")
    } 
}



fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Happy Error");
    let input = input.trim();

    match input_to_enum(input) {
        Some(state) => print_state(state),
        None => println!("Error")
    }

}