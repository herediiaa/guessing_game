use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    println!("Welcome to Guess the Number Game");

    let secret_number = rand::thread_rng().gen_range(1..=100); 
    //genera numero aleatorio
    println!("el numero aleatorio es {secret_number}");


    loop {
        println!("Please enter a number: ");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("failed");
        println!("{}", number);
        println!("{}", number.trim());
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match number.cmp(&secret_number) {
            Ordering::Less => println!("el numero es muy bajito"),
            Ordering::Greater => println!("el numero es muy grande"),
            Ordering::Equal => {
                println!("adivinaste muy bien");
                break;
            }
    

        }
}
}