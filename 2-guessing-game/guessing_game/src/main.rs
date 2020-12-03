use rand::Rng;
use std::io; // Standard Library, similar to those in C/C++
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // "let mut" := mutable variable (i.e. can be changed)

        io::stdin()
            .read_line(&mut guess) // "&mut" means a reference to a mutable variable; if not mutable, just &
            .expect("Failed to read line"); // Throws if 'read_line' returns a Result that's an 'Err' (error)

        // Shadowing the variable guess; avoids assigning a new number to another block of memory
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch-all value i.e. catches ALL errors
        };
        // trim gets rid of the implied `\n`
        // parse converts to a number

        println!("You guessed: {}", guess);

        // match - control flow based on pattern matching;
        // Here we compare `guess` to `secret number`
        // https://doc.rust-lang.org/std/keyword.match.html
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}