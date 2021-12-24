#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
       Coin::Penny => {
        println!("lucky penny!");
        1
       },
       Coin::Nickel => 5,
       Coin::Dime => 10,
       Coin::Quarter(state) => {
           println!("State quarter from {:?}!", state);
           25
        } 
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
       None => None,
       Some(i) => Some(i + 1), 
    }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alabama));

    println!("Hello, world!");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     other => move_player(other),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     _ => reroll(),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn reroll() {}

// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     _ => (),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
