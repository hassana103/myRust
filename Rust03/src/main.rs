use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the number!");
     // to be able to make randon number
    // we need to add a dependency in cargo.toml file
    // something like this 'rand = "0.5.5"'
    // after adding a dependency you should build your project
    // if you are in iran you should use shecan
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("the secret number is: {}",secret_number);

    loop{
        println!("Enter your guess:");

        // to define a new variable we use 'let' keyword
        // in rust the variables are emutable by default
        // it means you cant change them later
        // so if you want a variable to be mutable you can use 'mut' keyword
        // String is a data type in rust library
        // new is an associated function something like static function
        let mut guess = String::new();

        // to get the user input we need to use io library 'use std::io;'
        io::stdin().read_line(&mut guess).expect("failed ...");

        //parse the string to number
        // if the parse goes wrong continue
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);


        // now its time to compare to thing
        // first you need to use sdt::cmp::Ordering
        // the you can use cmp method like this
        // guess.cmp(&secret_number)
        // cmp get a reference to what you want to compare to 
        // cmp returns an Ordering object which you can use match like this for every posible situation
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","too small".yellow()),
            Ordering::Equal => {
                println!("{}" ,"you win!".green());
                break;
            },
            Ordering::Greater => println!("{}","too big".red()),
        }

    }

}
