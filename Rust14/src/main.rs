
// imagine you have a method like this
// this method return a reference to a string object
// but there is a problem
fn dangle() -> &String {

    let s = String::from("CYT"); // we define s here
    // so the scope of s ends after ending this method
    // so s is not valid after the method call
    // but here we want to return a reference to s(something that does not exist anymore)
    return &s; //then or reference point to nothing and we cant use it 
    // fotunatlly Rust wont let you compile this and gives you an error
    // and this is why this example wont compile
}

fn main() {
    // Dangling References

    // even if we could define dangle method like above
    // after calling it and catching the return value like this
    let a = dangle(); // a had no use to us because the value that a refers to nolonger exists

    println!("Hello, world!");
}
