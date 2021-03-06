extern crate getopts;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <command>", program);
    print!("{}", opts.usage(&brief));

    println!("\nCommands:");
    println!("    add <stuff>\t\tadd <stuff> to your todo list");
    println!("    remove <index>\tremove the item at <index> from your todo list");
    println!("    list\t\tlist your todo list");
}

fn main() {
    println!("rustickd v{}", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let (command, input) = if !matches.free.is_empty() {
        let mut cloned_matches = matches.free.clone();
        (cloned_matches.remove(0), cloned_matches.join(" "))
    } else {
        print_usage(&program, opts);
        return;
    };

    let path = Path::new("todo_list");
    let display = path.display();

    // create the todo list if it doesn't exist
    if !path.exists() {
        match File::create("todo_list") {
            Ok(file) => file,
            Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        };
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_) => &file_contents,
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
    };

    let mut list: Vec<String> = vec![];

    for line in file_contents.lines() {
        list.push(line.to_string());
    }

    println!("You have {} things on your todo list", list.len());

    println!("");

    match &*command {
        "add" | "a" => {
            println!("Adding '{}' to your todo list", input);
            list.push(input.to_string());
        }

        "remove" | "r" => {
            let index: usize = match input.parse() {
                Ok(result) => result,
                Err(_) => panic!(print_usage(&program, opts)),
            };

            if index <= list.len() {
                println!("Removing '{}' from your todo list", list[index - 1]);
                list.remove(index - 1);
            } else {
                println!("That's not a thing");
            }
        }

        "list" | "l" => {
            println!("Your todo list: ");

            for (index, item) in list.iter().enumerate() {
                println!("{}. {}", index + 1, item);
            }
        }

        _ => println!("I don't know what {} is :(", command),
    }

    let mut file = match File::create("todo_list") {
        Ok(file) => file,
        Err(why) => panic!("Couldn't save todo list {}", why.description()),
    };

    for line in list {
        let formatted_line = format!("{}\n", line);

        match file.write_all(formatted_line.as_bytes()) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't write {}: {}", display, why),
        };
    }
}
