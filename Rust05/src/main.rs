fn main() {

    //call the other function
    new_func();

    //call the funtion with parameters
    new_func_with_parameter(8 , 88);

    // call the faunction and get the return value
    let sum = new_func_with_return_value(10, 20);
    println!("the sum is: {}" , sum);
}

// define a new function using keyword 'fn'
fn new_func(){
    println!("another function ...");
}

// function with parameters
// parameters should be annotated - meaning their type should be defined
fn new_func_with_parameter(a:i32 , b:i32){
    println!("first parameter is {}" , a);
    println!("second parameter is {}" , b);
}


// function with return value
// after () use -> then return type
fn new_func_with_return_value(a:i32 , b:i32) -> i32 {
    let sum = a + b;

    // to return a value you can use 'return' keyword
    // return sum

    // or
    // you can simply give the expresion you want to return
    // for example
    // sum (you dont even need the ;)
    // or
    a+b
}
