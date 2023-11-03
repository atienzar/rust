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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        }
    }
}

fn main(){

        // if let tambien permite un ELSE
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }

        //OTRA FORMA DE HACERLO pero usando match

        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
}