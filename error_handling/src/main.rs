// use error_handling::handle_panic;
use error_handling::handle_error;

fn main() {
    handle_error::trigger_error();
    let name = handle_error::read_username_from_file();

    match name {
        Ok(s) => println!("name is {}", s),
        Err(e) => panic!("panic with {:?}", e)
    }
}
