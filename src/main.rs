// hi :3
use clap::{self, Arg};
use colored::Colorize;
use libc;
use std::{env, fs::OpenOptions, io::Write};
// Here is for executing other functions
fn main() {
    if !check_root() {
        let args: Vec<String> = env::args().collect();
        let elevate = std::process::Command::new("sudo")
            .args(&args)
            .status()
            .expect("elevation error");
        if !elevate.success() {
            std::process::exit(1);
        }
        return;
    }
    if !check_module() {
        return;
    }
    let matchy = clap::Command::new("thinkfan-cli")
        .version("0.1.2")
        .about("A Versatile Fan Control Utility for ThinkPad")
        .author("rahmedi rahmedyev@gmail.com")
        .arg(
            Arg::new("set")
            .short('s')
            .long("set")
            .required(false)
            .help("Set fan rate\nAvailable commands:\n1~7: is fan levels\nauto: automatic mode\nfull-speed: sets fan to full-speed\extreme: sets fan to its maximum speed\nenable: enables fan control\ndisable: disables fan control\n\nExamples:\nthinkfan-cli -s 7\nthinkfan-cli -s disengaged"))
        .arg(
            Arg::new("fetch")
            .short('f')
            .long("fetch")
            .action(clap::ArgAction::SetTrue)
            .help("Fetch's fan status")
        ).get_matches();

    if matchy.get_flag("fetch"){
        fetch();
        return;
    }

    let userinput = match matchy.get_one::<String>("set") {
        Some(set) => set.clone(),
        None => { 
            eprintln!("{}",format!("Invalid input! Try with -h").red());
            return;
        }
    };
    let commandlist = vec![
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "auto",
        "extreme",
        "full-speed",
        "enable",
        "disable",
    ];

    let modified_input = match commandlist.iter().any(|cmd| userinput.starts_with(cmd)) {
        true => format!("level {}", userinput),
        false => {
            eprintln!("{}", format!("Invalid Option: {}", userinput).red());
            eprintln!("\n{}", "Valid options are:".yellow());
            eprintln!("  0-7         : Fan levels");
            eprintln!("  auto        : Automatic mode");
            eprintln!("  full-speed  : Full speed");
            eprintln!("  extreme  : Maximum speed");
            eprintln!("  enable      : Enable fan control");
            eprintln!("  disable     : Disable fan control");
            eprintln!("\n{}", "Examples:".yellow());
            eprintln!("  thinkfan-cli -s 7");
            eprintln!("  thinkfan-cli -s auto");
            eprintln!("  thinkfan-cli -s extreme");
            return;
        }
    };
    let inputbool = match modified_input.contains("enable") || modified_input.contains("disable") {
        true => modified_input.replace("level ", ""),
        false => modified_input.clone(),
    };
    fan_level(inputbool);
}

// We're gonna check the fan control file to avoid errors.
fn check_file() -> bool {
    let fan_path = "/proc/acpi/ibm/fan";
    std::path::Path::new(fan_path).exists()
}

fn fetch() {
    match std::fs::read_to_string("/proc/acpi/ibm/fan") {
        Ok(content) => content.lines().take(3).for_each(|line| println!("{}", line)),
        Err(_) => eprintln!("{}", format!("Reading failed").red()),
    }
}

// We're using the control file for setting fan levels
fn fan_level(level: String) {
    let fan_path_true = match check_file() {
        true => "/proc/acpi/ibm/fan",
        false => {
            let error = format!("Error, Control file missing.").red();
            eprintln!("{}", error);
            return;
        }
    };

    let mut fan = OpenOptions::new()
        .write(true)
        .open(fan_path_true)
        .expect("Failed 71:1");

    fan.write_all(level.as_bytes()).expect("Writing error!");
}

fn check_module() -> bool{
    match std::fs::read_to_string("/sys/module/thinkpad_acpi/parameters/fan_control"){
        Ok(content) => {
            let content = content.trim();
            if content == "Y" {
                true
            }else if content == "N"{
                eprintln!("Hey, did you enable the thinkpad_acpi module? It appears you didn't.");
                false
            }else {
                println!("Unknown value {} :(", content);
                false
            }
        }Err(e) => {
            eprintln!("Failed to read the file, are you using a ThinkPad? (line 133) ({})", e);
            false
        }
    }
} 

fn check_root() -> bool {
    unsafe { libc::getuid() == 0 }
}
