use std::io;

fn main() {
    // TODO: load this from a file
    let mut list: Vec<String> = vec![];

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

        println!("Adding '{}' to your todo list", input);
        list.push(input)
    }
}

fn print_list(list: &Vec<String>) {
    println!("");
    println!("Your todo list: ");

    for (index, item) in list.iter().enumerate() {
        println!("{}. {}", index + 1, item);
    }
}
