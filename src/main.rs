use std::process;
use std::io;
fn main() {
    println!("Welcome to the guessing game! \n Guess a number from 1 to 1000. \n ready to begin?Y/n  ");
    let mut reply = String::new();
    io::stdin().read_line(&mut reply).expect("Error: Failed to read line !");
    println!("Your response was : {}",reply );
    if reply == "Y"{
        println!("Okay then");
        let mut isgamerunning = true;
        while isgamerunning == true {
            println!("game is running");
        }
    }else if reply == "n"{
        println!("Exiting");
        process::exit(1);
    }
}
