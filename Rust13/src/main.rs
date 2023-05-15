fn main() {
    // mutable references restrictions
    // 1. you can only have one mutable restriction at a time
    let mut s1 = String::from("hello1");

    let s11 = &mut s1; // we dont have any problem so far
    // but if we try to add another mutable reference like next line we will have problem
    // let s12 = &mut s1;
    
    // 1.1 actualy you cant have any other reference here not only mutable references
    // and that is because 's11' is still valid here
    // so the next line will make an error too
    // let s13 = &s1;

    println!("print references1: {} " , s11);

    // #####################################################################
    // but the references will go out of scope after their last use
    // for example we used 's11' in line 15 for the last time
    // because of that we can have another mutable reference OR as many simple references as we like (by simple references i mean notmutable references) 

    let s12 = &mut s1; // one mutable reference
    // OR
    // let s13 = &s1; // if we did not have line 22 we could have this 2 lines or many more like this
    // let s14 = &s1


    // describing more
    let mut s2 = String::from("hello2");

    // so if i make a reference of s2 here
    let s21 = &s2;
    // and use it here for the last time
    println!("print references2: {}" , s21); // s21 goes out of scope after this line
    // so we can have a mutable reference here(after last use)
    let s22 = &mut s1;

}
