extern crate rpassword;
extern crate password_generator;


use password_generator::generate_password;
use std::io;
use std::io::prelude::*;
use rpassword::read_password;

fn main() {
    // used symbols
    let symbols = vec![
        vec!['0','1','2','3','4','5','6','7','8','9',],
        vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',],
        vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',],
        vec!['!','#','$','%','&','*','+',',','-','.',':',';','<','=','>','?','@','_','~',],
        ];
    // Input:
    print!("Input name: ");
    io::stdout().flush()
        .ok()
        .expect("Could not flush stdout"); //this is stupid
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .ok()
        .expect("Failed to read line");
    print!("Master password: ");
    io::stdout().flush()
        .ok()
        .expect("Could not flush stdout");
    // Generate password
    let pw = read_password().unwrap();
    // Output:
    let result = match generate_password(&(name.trim_right_matches('\n').trim_right_matches('\r')),&pw,symbols,16) {
        Err(why) => panic!("Could not create password: {}", why),
        Ok(s) => s,
    };
    println!("{}",result);
}
