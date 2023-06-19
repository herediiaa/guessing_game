use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("welcome to guessing game");
    let random = rand::thread_rng().gen_range(0,101);
    let mut vidas: i8 = 10;
    loop {
    println!("please enter a number: ");
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("error");  
    let guess: u32 =match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("error con tu input");
            continue;
        }
    };
        
    match guess.cmp(&random){
        Ordering::Less => {
            vidas = vidas - 1;
            println!("Ups el numero es muy bajo")
        },
        Ordering::Greater => {
            vidas = vidas - 1;
            println!("Ups el numero es muy alto")
        },
        Ordering::Equal => {
            println!("eeeeyyyy ganaste el numero era {random}");
            break;
            
        }
    
    };
    
    if vidas == 0{
        println!("te quedaste sin vidas ;(");
        break;
    }
    println!("tenes {vidas} vidas")
    
}
    
    
     
}
