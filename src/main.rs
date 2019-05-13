use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn usage() {
    let program = env::args().nth(0).unwrap();
    println!("Usage: {} <number>", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // FIXME: CLI parsing instead.
    if args.len() != 2 {
        usage();
        exit(1);
    }

    let arg = &args[1];

    // TODO : intelligently find the right file
    let path = Path::new("/sys/class/leds/chromeos::kbd_backlight");
    let brightness_file = path.join("brightness");
    let max_brightness_file = path.join("max_brightness");

    let amount: i32 = arg.parse().unwrap_or_else(|_| {
        eprintln!("Input invalid. Expected a number.");
        exit(1);
    });

    let max_str = fs::read_to_string(max_brightness_file).unwrap();
    let max: i32 = max_str.trim().parse().unwrap();

    if amount < 0 || amount > max {
        usage();
        eprintln!("Keyboard expects between 0 and {}.", max);
        exit(1)
    }

    fs::write(brightness_file.clone(), arg).unwrap_or_else(|err| {
        eprintln!(
            "Unable to write to file '{}'\n{}",
            brightness_file.to_str().unwrap(),
            err
        );
        exit(1);
    });
}
