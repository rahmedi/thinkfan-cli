// Begin of Code
use libc;
use std::{env, fs::OpenOptions, io::Write};

// Colors
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

// Here is for executing other functions
fn main() {
    if !check_root() {
        let args: Vec<String> = env::args().collect();
        let elevate = std::process::Command::new("sudo")
            .args(&args)
            .status()
            .expect("Elevation error");
        if !elevate.success() {
            std::process::exit(1);
        }
        return;
    }
    if !check_module() {
        return;
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str(){
        "-f" | "--fetch" => {
            fetch();
        }
        "-s" | "--set" => {
            if args.len() < 3 {
                eprintln!("{}Undefinded Level{}",RED,RESET);
                return;
            }
            fanlogic(&args[2]);
        }
        "-h" | "--help" => {
            help();
        }
        _ => {
            help();
        }
    }

}

// Help function, a classic obviously
fn help(){
    println!("{}Thinkfan-cli v0.1.3{}",YELLOW,RESET);
    println!("Usage: thinkfan-cli [OPTIONS]");
    println!("\nOptions:");
    println!("  {}-s, --set <LEVEL>{} Sets fan rate",YELLOW,RESET);
    println!("\n    Available commands are:");
    println!("          1~7         : Fan levels");
    println!("          auto        : Automatic mode controlled by EC");
    println!("          full-speed  : Sets fan to its secure maximum speed");
    println!("          disengaged  : Overspeeds fan (Warning!)");
    println!("          enable      : Enables fan control");
    println!("          disable     : Disables fan control");
    println!("          ");
    println!("          Examples:");
    println!("          thinkfan-cli -s 7");
    println!("          thinkfan-cli -s disengaged");
    println!("          ");
    println!("  {}-f, --fetch{}             Fetch's fan status",YELLOW,RESET);
    println!("  {}-h, --help{}              Print Help",YELLOW,RESET);
    println!("  {}-V  --version{}           Print Version",YELLOW,RESET);
}

fn fanlogic(userinput: &str){
    let commandlist = vec!["0","1","2","3","4","5","6","7","auto","disengaged","full-speed","enable","disable"];

    if !commandlist.iter().any(|&cmd| userinput == cmd) {
        eprintln!("{}Invalid Option{}",RED,RESET);
        return;
    }

    let level = if userinput == "enable" || userinput == "disable" {
        userinput.to_string()
    }else {
        format!("level {}", userinput)
    };

    fan_level(level);
}
// We gonna check fan control file for reducing errors
fn check_file() -> bool {
    let fan_path = "/proc/acpi/ibm/fan";
    std::path::Path::new(fan_path).exists()
}

fn fetch() {
    match std::fs::read_to_string("/proc/acpi/ibm/fan") {
        Ok(content) => content.lines().take(3).for_each(|line| println!("{}", line)),
        Err(_) => eprintln!("{}Read Failure{}",RED,RESET),
    }
}

// We interacting with control file for setting fan levels
fn fan_level(level: String) {
    let fan_path_true = match check_file() {
        true => "/proc/acpi/ibm/fan",
        false => {
            eprintln!("{}Control File is not available{}",RED,RESET);
            return;
        }
    };

    let mut fan = OpenOptions::new()
        .write(true)
        .open(fan_path_true)
        .expect("Critical: ThinkPad ACPI communication failed");

    fan.write_all(level.as_bytes()).expect("Writing error!");
}

fn check_module() -> bool{
    match std::fs::read_to_string("/sys/module/thinkpad_acpi/parameters/fan_control"){
        Ok(content) => {
            let content = content.trim();
            if content == "Y" {
                true
            }else if content == "N"{
                eprintln!("Hey, did you enabled the thinkpad_acpi module? Seems like you didnt.");
                false
            }else {
                println!("Unknown value {} :(", content);
                false
            }
        }Err(e) => {
            eprintln!("Failed to read file, are you using a thinkpad? ({})", e);
            false
        }
    }
}

fn check_root() -> bool {
    unsafe { libc::getuid() == 0 }
}
// End of Code
