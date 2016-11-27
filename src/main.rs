use std::io;

fn main() {
    println!("rustickd v0.1.0");
    println!("You have {} things on your todo list", 0);

    loop {
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

        println!("You said: {}", input);
    }
}
