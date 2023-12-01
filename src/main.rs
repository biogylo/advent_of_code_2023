mod friend;
use crate::friend::Friends;

fn main() {
    println!("Hello, world!");
    let amigo = Friends::Hector;
    let friends: String = amigo.to_string();
    println!("{}", friends);
}
