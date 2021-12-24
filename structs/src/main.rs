struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("lets talk about structs");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    let _user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    println!("user email: {:?}", user1.email);

    let mut user_mutable = User {
        email: String::from("mutable@example.com"),
        username: String::from("mutable"),
        active: true,
        sign_in_count: 1,
    };

    user_mutable.active = false;

    let _new_builder = build_user(String::from("bla@bla.com"), String::from("bla"));

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    println!("user email: {:?}", black.0);

    // unit-like structs
    struct AlwaysEqual;

    let _subject = AlwaysEqual;
}
