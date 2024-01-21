// {use} keyword helps to import the libraries
// {std} is the standard library
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    // Variable name should be snake case
    let scret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess below");

        // {mut} keyword makes the variable to be mutable.
        // {String::new()} - cerates the empty string without any hash in memory allocation, which makes the computation inexpensive
        // I used the guess variable inside the loop because - while executing the code for the 2nd input - throwing an error
        //      - because we have referenced the value.
        let mut guess = String::new();

        // 1) io - input and output
        // 2) stdin - to take the input from the user
        // 3) We can also use like std::io::stdin if we did not imported std ::io at top
        // 4) read_line - will get the data from the user and append them with the argument we passed.
        // 5) "&"" indicates that the reference of the variable - so the same variable can be refered accross project
        //      the referenced data will not be copied into the memory.
        // 6) read_line responds with "Result" that is enumeration - means enums - which results in one ore more possible output
        //      so its better to handle those values
        io::stdin()
            .read_line(&mut guess)
            .expect("Handling the failure message");

        let guess_number_format: u32 = guess.trim().parse().expect("please enter the number");

        match guess_number_format.cmp(&scret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // "{}" implies that this is not static text (string literal)
    // println!("Your Guess: {guess}");
}
