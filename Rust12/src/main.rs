fn main() {

    // what if we want to modify the borrowed value
    // we need to use mutable references

    // first we need to declare the base variable mutable
    let mut s = String::from("hello");

    // now we need a method that takes a mutable reference like metho2 here
    // and call it but this time we pass a mutable reference
    method2(&mut s);

    // now we print s to see the change
    println!("s is changed s:{}" , s);

    
    // same goes for assigning new values
    let mut s2 =String::from("new text");

    let s3 = &mut s2;

    // now we change s3
    s3.push_str(" pushed");

    println!("we still have access to s2: {}" , s2);
    
}

fn method2(s: &mut String){ //if you remember we just had '&' in last example but here we have '&mut'
    // now here we can change s
    s.push_str(" world");

}
