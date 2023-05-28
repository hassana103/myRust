fn main() {
    // tuple structs
    // it is like a struct but fields dont have names and only types

    let c1 = Color(10 , 11 , 12);
    let car1 = Car(String::from("BMW") , 20);

    println!("{}" , c1.2);
    println!("{}", car1.0);
}

struct Color(i32 , i32 , i32);
struct Car(String , i32);


