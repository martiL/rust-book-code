// hello, world
fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    another_function(5, 'h');
    println!("five function return value {}", five());
    println!("5 + 1 is: {}", plus_one(5));
}

fn another_function(value: i32, unit_label: char) {
    println!("the value of x is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
fn plus_one(x: i32) -> i32 {
    x + 1 // I’m feeling lucky today
}
