use std::io;
fn main() {
    let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Err reading stdin");

    println!("You wrote: {}",user_input);

}
