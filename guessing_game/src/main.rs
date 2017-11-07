extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
//use std::result;
//use std::string;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut cnt = 0;
    loop {
        println!("Please input your guess.");
        let guess : u32 = match line_input() {
            Ok(num) => {num}
            Err(e) => {println!("{}", e); continue;}
        };


        println!("You guessed: {}", guess);
        cnt +=1;
        println!("Number of tries: {}", cnt);
        match guess.cmp (&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
            }
        }
    }

}
fn line_input() -> Result<u32,String>  {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

        match guess.trim().parse() {
        Ok(num) => return Ok(num),
        Err(e) => return Err(String::from(format!("in parsing u32, {}", e))), //println!("ERROR: Non integer input : {}", e);  continue;}
    };

}
