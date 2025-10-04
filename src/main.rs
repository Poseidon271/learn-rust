use std::io;

fn main()
{
    println!("Guess the number");
    println!("Please input your guess");
    let mut num1 = String::new();
   // let mut num1:u8;
     io::stdin()
     .read_line(&mut num1)
     .expect("Failed to read line");
     println!("correct :{num1}");
}
