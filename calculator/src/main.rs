use std::env::{args, Args};


fn operar(operacion: char, primer_operando: f32, segundo_operando: f32) -> f32 {
    match operacion {
        '+' => primer_operando + segundo_operando,
        '-' => primer_operando - segundo_operando,

        '/' => primer_operando / segundo_operando,
        '*' | 'x' | 'X' => primer_operando * segundo_operando,
        _ => panic!("Operador utilizado no válido."),
    }

}

fn main() {
    println!("Hello, world!");

    let mut args: Args = args();
    // Args { inner: ["target/debug/calculator", "prueba1", "prueba2", "prueba3"] }
    println!("{:?}", args);

    // nth itera sobre la variable args  y la va "cambiando" => va haciendo POP
    // unwrap convierte el objeto a String 
    let primero = args.nth(1).unwrap();
    // Después de acceder al segundo argumento, el elemento iterador siguiente se convierte en el primero
    let operador: String = args.nth(0).unwrap();
    let segundo: String = args.nth(0).unwrap();
  
    println!("{} {} {}", primero, operador, segundo);

    // primero y segundo deben pasar de Strings a f32 

    let primer_operando = primero.parse::<f32>().unwrap();
    let segundo_operando = segundo.parse::<f32>().unwrap();

    println!("{} {} {}", primer_operando, operador, segundo_operando);

    let operacion = operador.parse::<char>().unwrap();

    let resultado = operar(operacion, primer_operando, segundo_operando);
    println!("El resultado es {}", resultado);
}
