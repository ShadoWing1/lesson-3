use std::io::{self, Write};

fn input(input_entry: &str) -> String {
    let mut input: String = String::new();
    print!("{}: ", input_entry);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap(); // input input
    input.pop(); // popping last character '\n'
    return input;
}

fn main()
{
    let user_name: String = input("name");
    let user_age: String = input("age");
    let user_status: String = input("status");

    let _num: i32 = user_age.trim().parse::<i32>().expect("Lütfen sayı giriniz");

    println!("Hello {}.Your age {} and your status {}.", user_name ,_num ,user_status);
}
