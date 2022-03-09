use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("                                      ");
    println!("                                       ");
    println!("___________________GUESS THE NUMBER_________________");
    println!("                                      ");
    println!("                                       ");
  
    
    loop {
        let secret_number = rand::thread_rng().gen_range(1..11);
        println!("Enter guess a number in between 1 to 10");
        let mut guess = String::new();
   
        io::stdin()
            .read_line(&mut guess)
            .expect("Error to read_line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!(" Your guess is :{} and the secret number is: {}",guess,secret_number);// --snip--
        println!("                       ");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("you lose!!Too small!"),
            Ordering::Greater => println!("you lose!!Too big!"),
            Ordering::Equal =>{ println!("Congrats!!!You win!!!");
            
            break;
        }
       
        
        }
        println!("                       ");
        println!("Type quit for QUIT GAME");
        println!("        OR   ")
    }

}
