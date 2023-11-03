extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    println!("¡Bienvenido a Advina el número! ");
    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    //println!("Eñ múmero secreto es {} ",numero_secreto);
    

    loop{
        println!("Introduce tu corazonada ");
        

        let mut intento = String::new();
        
        io::stdin().read_line(&mut intento)
            .ok()
            .expect("Error al leer la línea.");

        println!("Tu corazonada fue {} ", intento);

        let intento: u32 = match intento.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
                    

        match intento.cmp(&numero_secreto) {
            Ordering::Less    => println!("Muy pequeño!"),
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Equal   => {
                println!("Has ganado!");
                break;
            }
        }
    }
}
