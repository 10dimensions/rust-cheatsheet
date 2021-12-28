
//1. variables
fn variables(){
    let mut x = 1;  //re-assignable
    println!("The value of x is: {}", x);
    x = 0;
    println!("The value of x is: {}", x);
}

//2. datatypes
fn datatype(){
    let age: u8 = 18; //Unsigned integer
    println!("You are {} years old.", age);

    let b = true; //boolean
    println!("It is {}", b);

    let c = "Karl Marx is right!";
    println!("{}", c);
}

fn stringtype(){

}

fn main() {
    println!("Hello, world!");

    datatype()
}
