enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usState),
}
#[derive(Debug)]
enum usState{
    Alabama,
    Arkansas,
}


fn main() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!(" five es {:?} six es {:?} y none es {:?}",five,six,none);


    // Estas 2 formas hacenn lo mismo.

    let config_max = Some(3u8);
    //let config_max = Option::None;
    //let config_max: Option<T> = Option::None;
    match config_max { 
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Aqui usamos if let que asigna la vbnle si tiene valor.
    let config_max = Some(7u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }


    // if let tambien permite un ELSE
    let mut count = 0;
   // if let Coin::Quarter(state) = coin {
   //     println!("State quarter from {:?}!", state);
   // } else {
   //     count += 1;
   // }
}


fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1)
    }
}

