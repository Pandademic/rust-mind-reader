use std::io;
use std::process;
fn main() {
    println!("Welcome to the guessing game! \n Guess a number from 1 to 1000. \n ready to begin?Y/n  ");
    let mut reply = String::new();
    io::stdin().read_line(&mut reply).expect("Error: Failed to read line !");
    if reply == "Y"{
        //continue
        println!("Continuning...");
    }else{
      process::exit(1);  
    }
}
