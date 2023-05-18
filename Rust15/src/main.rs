fn main() {

    // a more complex situation in borrowing and slices
    //first we define a String
    let mut s = String::from("CYT");

    // now we borrow s and give it to our method (simple reference)
    // here we let the method borrow s but the method return a reference
    // Rust does not care what reference the mothod returns, it just cares that it borrowd s and returnd a reference
    // so it thinks that r borrowed s and untill r is not out of the scope it does not let you have a mutable reference
    let r = return_reference(&s);
    // because of that here we cant change s and use r in the next line
    s.push_str("HA"); // look for 'push_str' or 'clear' method and you see they take a mutable reference of their caller
    println!("Hello, world!{}",r); // if we dont use r here, this code will compile and run!!!

    // this may look complicated but its just like before
    let mut s2 = String::from("Leon");

    // first we take a simple reference
    let r1= &s2;
    // actually we can as many simple reference as we want
    let r2 = &s2;

    // but if we want to have a mutable reference it should be before simple references or after their last use
    // for example we cant have a mutable reference here
    let r3 = &mut s2;
    // and then use the simple references here
    println!("if we use r1 or r2 here, this code wont compile {}" , r2);
    // try to change it the way that it compiles

}

fn return_reference(s:&String) -> &str{

    let s2 = "another text";

    &s2

}
