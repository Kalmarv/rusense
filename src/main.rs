use clap::{arg, Command};
use rand::Rng;

fn main() {
    let matches = Command::new("Random Sensitivity Generator")
        .version("0.1.0")
        .author("Kalmarv")
        .about("Generates random sensitivity for games")
        .arg(arg!(--min <i32>).required(true))
        .arg(arg!(--max <i32>).required(true))
        .get_matches();

    let min: f64 = matches.get_one::<String>("min").unwrap().parse().unwrap();
    let max: f64 = matches.get_one::<String>("max").unwrap().parse().unwrap();

    let val_option = gen_cm_range(min, max);
    match val_option {
        Some(val) => println!("Your sensitivity is: {}", val),
        None => println!("Max must be greater than min"),
    }
}

fn gen_cm_range(min: f64, max: f64) -> Option<f64> {
    if min >= max {
        return None;
    }
    let mut rng = rand::thread_rng();
    let rand: f64 = rng.gen_range(min..=max);
    return Some(rand);
}
