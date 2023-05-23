struct User{
    name: String,
    email: String,
    is_active: bool,
    login_count: u32,
}

fn main() {
    // we can have functions to take parameters and make new instance for us
    let user1 = build_user(String::from("Leon"), String::from("Leon@cyt.ir"));

    println!("name of user1: {}" , user1.name);

    let user2 = build_user_shorthand(String::from("Bob"), String::from("bob@cyt.ir"));

    println!("name of user2: {}", user2.name);

}

fn build_user(a: String , b:String) -> User {
    User{ 
            name: a,
            email: b,
            is_active: true,
            login_count: 1 
        }
}

fn build_user_shorthand(name:String , email:String) -> User{
    // if the parameter and the field have the same name we can use shorthand syntax
    User{
        name,
        email,
        is_active: true,
        login_count:1
    }
}


