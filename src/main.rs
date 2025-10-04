use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main()
{
    println!("Guess the number");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("the secret number is : {secret_num}");
    loop{
    println!("Please input your guess");
    let mut num1 = String::new();
   // let mut num1:u8;
     io::stdin()
     .read_line(&mut num1)
     .expect("Failed to read line");
     let num1: u32 = match num1.trim().parse() {
        Ok(num) => num,
    Err(_) => continue,
     };
      println!("You guessed: {num1}");
     match num1.cmp(&secret_num){
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
             println!("You Win");
             break;
        }
     }
    }
}
