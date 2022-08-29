use std::io;

enum PowerState{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState{
    fn new(state: &str) -> Option<PowerState>{
        let state = state.trim().to_lowercase(); // .to_lowercase() converts string slice to String
        // .as_str() converts String -> &str and match needs same type
        match state.as_str(){
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }
}

fn print_power_state(state: PowerState){
    use PowerState::*;
    match state{
        Off => println!("turning off"),
        Sleep => println!("sleeping"),
        Reboot => println!("Rebooting.."),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("going in Hibernation"),
    }
}

fn main() {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer); //read_line returns result
    if user_input_status.is_ok(){ //is_ok() is defined in result
        match PowerState::new(&buffer){
            Some(state) => print_power_state(state),
            None => println!("Invalid power state"),
        }
    }else{
        println!("Error reading input");
    }
}
