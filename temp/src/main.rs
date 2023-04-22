use std::io;

fn main() {
    println!("Welcome to temperature converter+!");

    let c_to_f_selected: bool = loop {
        println!("What conversion would you like?");
        println!("[C] Fahrenheit to Celcius");
        println!("[F] Celcius to Fahrenheit");

        let mut desired_deg = String::new();

        io::stdin()
            .read_line(&mut desired_deg)
            .expect("Failed to read value");

        let desired_deg = desired_deg.trim();

        match desired_deg {
            "F" | "C" => {
                println!("You selected {desired_deg}");
                break desired_deg == "F";
            }
            _ => {
                println!("Invalid selection");
                continue;
            }
        }
    };

    println!("Enter the temperature");

    let mut initial_temp = String::new();

    io::stdin()
        .read_line(&mut initial_temp)
        .expect("Failed to read temperature");

    let initial_temp: f64 = initial_temp
        .trim()
        .parse()
        .expect("Failed to parse");

    let final_temp = if c_to_f_selected { 
        c_to_f(initial_temp) 
    } else { 
        f_to_c(initial_temp) 
    };

    println!("{final_temp}");
}

fn c_to_f(c_deg: f64) -> f64 {
    c_deg * (9.0 / 5.0) + 32.0
}

fn f_to_c(f_deg: f64) -> f64 {
    (f_deg - 32.0) * (5.0 / 9.0)
}