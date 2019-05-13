use std::fs;
use std::env;
use std::path::{Path};

fn usage() {
    let program = env::args().nth(0).unwrap();
    println!("Usage: {} <number>", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // FIXME: CLI parsing instead.
    if args.len() != 2 {
        usage();
    ::std::process::exit(1)
    }

    let arg = &args[1];

	// TODO : intelligently find the right file
    let path = Path::new("/sys/class/leds/chromeos::kbd_backlight");
    let brightness_file = path.join("brightness");
    let max_brightness_file = path.join("max_brightness");

	// FIXME: error handling for non-integers.
    let amount: i32 = arg.parse().unwrap();

	// TODO : find max_brightness
	let max_str = fs::read_to_string(max_brightness_file).unwrap();
	let max: i32 = max_str.trim().parse().unwrap();

    if amount < 0 || amount > max {
        usage();
        println!("Keyboard expects {} max.", max);
        ::std::process::exit(1)
    }

    fs::write(brightness_file.clone(), arg)
        .expect(&format!("Unable to write to file '{}'", brightness_file.to_str().unwrap()));
}
