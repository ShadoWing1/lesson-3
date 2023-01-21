use std::io;

fn main()
{
    let mut user_name = String::new();
    let mut user_age = String::new();
    let mut user_status = String::new();

    print!("Name: ");
    io::stdin().read_line(&mut user_name).unwrap(); // input input

    print!("age: ");
    io::stdin().read_line(&mut user_age).unwrap(); // input input
    let _num = user_age.trim().parse::<i32>().expect("Lütfen sayı giriniz");

    print!("status: "); //student ,doctor ,unemployed bla bla bla
    io::stdin().read_line(&mut user_status).unwrap(); // input input


    println!("Hello {}.Your age {} and your status {}.", user_name ,_num ,user_status);

}