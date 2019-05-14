use glob::glob;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

/// Returns the path representing the given sysfs ressource
/// for the first kbd_backlight found.
fn get_file(name: &str) -> PathBuf {
    let patt = "/sys/class/leds/*::kbd_backlight/";
    let dir = glob(patt)
        .unwrap()
        .nth(0)
        .unwrap_or_else(|| {
            eprintln!("Couldn't find a sysfs directory matching '{}'", patt);
            exit(2);
        })
        .unwrap();
    let base_path = Path::new(&dir);
    //let base_path = leds_path.join("chromeos::kbd_backlight");

    base_path.join(name)
}

/// Prints usage to stdout.
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

    let brightness_file = get_file("brightness");
    let max_brightness_file = get_file("max_brightness");

    let amount: i32 = arg.parse().unwrap_or_else(|_| {
        eprintln!("Input invalid. Expected a number.");
        exit(1);
    });

    let max_str = fs::read_to_string(max_brightness_file.clone()).unwrap_or_else(|err| {
        eprintln!(
            "Couldn't read file '{}'.\n{}",
            max_brightness_file.to_str().unwrap(),
            err
        );
        exit(2);
    });
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
