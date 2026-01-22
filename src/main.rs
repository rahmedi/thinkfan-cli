// Begin of Code
use std::{env, fs::OpenOptions, io::Write};

// Constant Variables
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const FANPATH: &str = "/proc/acpi/ibm/fan";
const HELP_MSG: &str = "\x1b[33mthinkfan-cli v0.1.4\x1b[0m
Usage: thinkfan-cli [OPTIONS]

Options:
  \x1b[33m-s, --set <VALUE>\x1b[0m      Sets fan rate
  \x1b[33m-f, --fetch\x1b[0m            Fetch fan status
  \x1b[33m-h, --help\x1b[0m             Print Help
  \x1b[33m-V, --version\x1b[0m          Print Current Version

Available values for -s:
  \x1b[33m1~7\x1b[0m                 Fan levels
  \x1b[33mauto\x1b[0m                Automatic mode controlled by EC
  \x1b[33mfull-speed\x1b[0m          Secure maximum speed
  \x1b[33mdisengaged\x1b[0m          Overspeed mode \x1b[31m(WARNING!)\x1b[0m
  \x1b[33menable\x1b[0m              Enable fan control
  \x1b[33mdisable\x1b[0m             Disable fan control";

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
        print!("{}\n", HELP_MSG);
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
            print!("{}\n", HELP_MSG);
        }
        "-V" | "-v" | "--version" => {
            println!("v0.1.4");
        }
        _ => {
            print!("{}\n", HELP_MSG);
        }
    }

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
    std::path::Path::new(FANPATH).exists()
}

fn fetch() {
    match std::fs::read_to_string(FANPATH) {
        Ok(content) => content.lines().take(3).for_each(|line| println!("{}", line)),
        Err(_) => eprintln!("{}Read Failure{}",RED,RESET),
    }
}

// We interacting with control file for setting fan levels
fn fan_level(level: String) {
    let fan_path_true = match check_file() {
        true => FANPATH,
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
    let uid: u32;
    unsafe {
        #[cfg(target_arch = "x86_64")]{
            // 64-bit systems
            std::arch::asm!(
                "mov eax, 102",
                "syscall",
                out("rax") uid,
            );
        }

        #[cfg(target_arch = "x86")]
        {
            // 32-bit systems
            std::arch::asm!(
                "mov eax, 199",
                "int 0x80",
                out("eax") uid,
            );
        }
    }
    uid == 0
}
// End of Code
