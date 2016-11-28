use std::io;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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

    println!("{}", file_contents);

    // TODO: load this from a file
    let mut list: Vec<String> = vec![];

    for line in file_contents.lines() {
        list.push(line.to_string());
    };

    println!("rustickd v0.1.0");
    println!("You have {} things on your todo list", list.len());

    loop {
        if list.len() > 0 {
            print_list(&list);
        }

        println!("");
        println!("Enter a command");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("???");

        let input: String = match input.trim().parse() {
            Ok(text) => text,
            Err(_) => continue,
        };

        if input == "quit" {
            println!("Bye!");
            break;
        }

        // split input on the first space, so command[0] is the command and
        // command[1] the "arguments"
        let command: Vec<&str> = input.splitn(2, ' ').collect();

        if command.len() < 2 {
            println!("Hey, no");
            continue;
        }

        // TODO: can't pattern match on vectors so figure out a nicer way to do
        // line 36 -> this line
        let (command, arguments) = (command[0], command[1]);

        match command {
            "add" => {
                println!("Adding '{}' to your todo list", arguments);
                list.push(arguments.to_string());
            }

            "remove" => {
                let index: usize = match arguments.parse() {
                    Ok(result) => result,
                    Err(_) => continue,
                };

                if index <= list.len() {
                    println!("Removing '{}' from your todo list", list[index - 1]);
                    list.remove(index - 1);
                } else {
                    println!("That's not a thing");
                    continue;
                }
            }

            _ => {
                println!("What?");
                continue;
            }
        }
    }
}

fn print_list(list: &Vec<String>) {
    println!("");
    println!("Your todo list: ");

    for (index, item) in list.iter().enumerate() {
        println!("{}. {}", index + 1, item);
    }
}
