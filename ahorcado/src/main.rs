use std::io;

fn actualizar_pista(pista: &mut Vec<char>, letra: char, secreto:Vec<char> )->&Vec<char>{
    for i in 0..secreto.len()-1{
        if secreto[i] == letra{
            pista[i] = letra
        }
    }
    pista
}

fn main() {
    println!("Bienvenido al juego del ahorcado!");

    let str_secreto = String::from("escoba");
    let pista :Vec<char> = String::from("______").chars().collect();
    let secreto: Vec<char> = str_secreto.chars().collect();
    let mut adivinadas: Vec<char>  = Vec::new();
    let mut jugadas: Vec<char>  = Vec::new();

    //let mut v = String::new();
    let mut encontrado:bool = false;

    while adivinadas.len() < secreto.len() {
        //println!('');
        println!("Introduzca una letra: ");
        
        let mut v = String::new();
        io::stdin()
        .read_line(&mut v)
        .expect("Error leyendo la linea.");

        let letra: Vec<char> = v.chars().collect();
        println!("Ahora v vale {:?}", v);
        //println!("Quiepre jugar con la letra {}", letra[0]);
        jugadas.push(letra[0]);


        for i in 0..str_secreto.len(){
            if secreto[i] == letra[0]{
                adivinadas.push(letra[0]);
                println!("¡Bien! la letra está" );
                pista = actualizar_pista(&mut pista, letra[0],secreto);
                encontrado = true;
                break
            }
        }
        println!("Ya ha jugado con {:?}",jugadas);

    }




}

