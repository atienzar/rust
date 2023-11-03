fn main() {

    //esto es inmutable
    let mut s = "Hello!";
    s ="Hola!";
    println!("{}",s);


    //Cadena de cadena,de esta forma se puede mutar
    let  mut s = String::from("hello");
    s.push_str(" ,world");
    println!("{}",s);

    let s1 = String::from("Hola");
    let s2 = s1;

    // Ahora tienes 2 punteros que apuntan a la mismas pos de mem
    // como no tiene sentido tener 2 punteros apuntando al mismo sitio
    // Rust se carga al primero!! 

    println!("{}, world!", s2);

    let s1 = s2.clone();
    println!("{}, copiadooo!", s2);



    let x = 5;
    let y = x;

}
