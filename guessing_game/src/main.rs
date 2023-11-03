use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number! ");

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");

    loop{

        println!("Please input yor guess:");

        // la vble nunca cambia de valor, a no ser que la hagas mutable. 
        //define 1 vble mutable called guess
        //sin embargo no pone tipo. 
        
        //String::new => es una funcion oasocida a un tipo, en este caso 
        // crea 1 nuevo string vacío. 

        let mut guess = String::new();

            
        //, references are immutable by default para que se puedan modificar 
        // hay q especificarlo por eso &mut guess

        //Result's variants are Ok and Err (tipo enum)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        //para forzar q sea un numero hacemos sombra a la anterior => Shadowing
        //Rust allows us to shadow the previous value of guess with a new one. 

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,  //si hay un error, se salta la ejecucion y sigue
        };

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    //println!("x = {} and y = {}", x, y);
}