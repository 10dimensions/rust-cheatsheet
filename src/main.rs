
//1. variables
fn variables(){
    let mut x = 1;  //re-assignable
    println!("The value of x is: {}", x);
    x = 0;
    println!("The value of x is: {}", x);
}

//2. numbers
fn numbers(){
    let age: u8 = 18;
    println!("You are {} years old.", age)
}

fn main() {
    println!("Hello, world!");

    numbers()
}
