fn main() {
    // we explained that methods can take the ownership of a value
    // if we want the method to give back this ownership we could make it to return the value
    // but this is not the right way
    // in other word, we look for a way to call a method an give it the parameters without giving it the ownership
    // in this sutuation we can pass a reference as parameter
    // a reference is like a pointer, a address that we can folow to acess data

    let s = String::from("hello");

    // now we call the method
    my_method(&s); //this time we use '&' to pass a reference

    // as we can see in next line, the ownership is not taken from s and s is still valid
    // so we can use it
    println!("s is still valid s: {}" , s);

    ///////////////////////////////////////////////////////////////
    //same goes for assigning variables to each other
    // here we assaign s2, a reference to s
    let s2 = &s;

    // but s2 does not take the ownership of the value
    // and s is still the owner of the value and VALID
    println!("s is still valid s: {}" , s);


    //========================================================
    //| we call the action of creating a reference BORROWING |
    //========================================================
    
    
}

// this method wont take ownership of its parameters
fn my_method(s:&String){ // the '&' here indicates that this method needs a reference
    println!("{}" , s);
}
