extern crate read_input;

mod expense;
mod fortnight;
mod register;

use std::io::Write;
use std::io::prelude::*;
use std::fs::OpenOptions;

use read_input::prelude::*;

use register::Register;

fn main() {
    let files = get_files();

    let file = choose_file(files);
    println!("Reading {}...", file);

    let mut register = Register::load_file(file);
    'shell: loop {
        let incmd: String = input().msg(format!("F[{}]:${} > ", register.get_i(), register.get_current().get_remaining() )).get();
        if incmd=="" {
            continue 'shell;
        }

        let (command, args) = get_command(incmd);
        
        match command.as_ref() {
            "add" => register.add_cmd(args),
            "edit" => register.edit_cmd(args),
            "inc" => register.increase_cmd(args),
            "ls" => register.list_cmd(args),
            "rm" => register.remove_cmd(args),
            "set" => register.set_cmd(args),
            "st" => register.step_cmd(args),
            "save" => register.save_in_file(),
            "ex" | "exi" | "exit" => { println!("Good bye!"); break 'shell; },
            _ => println!("Error: invalid command '{}'", command),
        }
    }
}

fn get_files() -> Vec<String> {
    let reg_def = "app/reg_DEFAULT_isters.txt";
    let mut files = Vec::new();

    println!("Starting...");

    let mut content = String::new();
    if let Ok(mut f) = OpenOptions::new().read(true).open(reg_def) {
        println!("Loading files...");
        f.read_to_string(&mut content).unwrap();
    } else {
        println!("Files register unavailable!\nCreating files register...");
		let mut file = OpenOptions::new().write(true).create(true).open(reg_def).unwrap();
        file.write_all(b"default.txt\n").unwrap();

        let mut file = OpenOptions::new().read(true).open(reg_def).unwrap();
        file.read_to_string(&mut content).unwrap();
		println!("Files register created!\nDefault file added!");
        
        OpenOptions::new().write(true).create(true).open("data/default.txt").unwrap();
    }

    for line in content.lines() {
        files.push(String::from(line));
    }

    files
}

fn choose_file(files: Vec<String>) -> String {
    if files.len() == 1 {
        String::from(files[0].as_ref())
    } else {
        'valid: loop {
            println!("Choose file to read... (ENTER for default.txt)");
            for (i, filename) in files.iter().enumerate() {
                println!("{}.- {}", i+1, filename);
            }

            let option: String = input().repeat_msg("> ").get();

            if option == "" {
                return String::from("default.txt");
            } else {
                let n = match option.parse::<usize>() {
                    Ok(n) => n,
                    Err(_e) => 0,
                };

                if n>0 && n<=files.len() {
                    return String::from(files[n-1].as_ref());
                } else {
                    println!("Invalid option, try again!");
                }
            }
        }
    }
}

fn get_command(input: String) -> (String, Vec<String>) {
    let mut args = Vec::new();

    let mut splited = input.split_whitespace();

    let cmd = String::from(splited.next().unwrap());

    for arg in splited {
        args.push(String::from(arg));
    }

    (cmd, args)
}