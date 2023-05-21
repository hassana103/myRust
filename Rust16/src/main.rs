fn main() {
    // String slices
    let mut s = String::from("Hello CYT");

    // the point is when you take a slice you are actually borrowing the value
    let s1 = slices(&s);

    // so you cant have a mutable borrow later and before last use of simple borrow
    s.clear(); // a mutable borrow happens here

    // last use of slice
    println!("printing the slice: {}" , s1);
    

    // #########################################################to make it simple
    let mut s20 = String::from("hello CYT 2");

    // simple borrowing
    let s21 = &s20[..];

    // mutable borrow
    s20.push_str("pushed string");

    //last use of simple borrow
    println!("printing the slice 2 {} ",s21);

    // if we remove the mutable borrows this code will work!!!
}

fn slices(s:&String) -> &str {
    //we can have a slice of string like this
    let s1 = &s[0..5]; // [a..z] is a range a is inclusive and z is exclusive
    
    // if you want your slice to start from the beggining of the string
    let s2 = &s[..5]; // you can remove a

    // if you want your slice to end at the end of your string
    let s3 = &s[2..]; // you can remove z

    // if you want all of your string
    let s4 = &s[..]; // you can remove a and z

    return &s4;
}
