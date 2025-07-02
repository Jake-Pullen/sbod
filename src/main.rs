use clap::Parser;

#[derive(Parser)]
struct Cli {
    current_speed: i64,
    target_speed: i64,
}

fn main() {
    let args = Cli::parse();
    let target_armour = work_out_armour(args.current_speed, args.target_speed);
    println!(
        "Current Speed: {}, Target Speed: {}",
        args.current_speed, args.target_speed
    );
    println!("Armour Required to get to Target Speed: {}", target_armour);
}

fn work_out_armour(current_speed: i64, target_speed: i64) -> f64 {
    let offensive_defence_multiplier = -0.25;
    (target_speed as f64 / offensive_defence_multiplier)
        - (current_speed as f64 / offensive_defence_multiplier)
}
