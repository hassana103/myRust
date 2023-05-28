fn main() {
    //Struct Update Syntax

    // we have a user instance
    let user1 = User{
        name: String::from("Leon"),
        email: String::from("leon@cyt.ir"),
        is_active: true,
        login_count: 10
    };

    // now we want to create new users using user1 
    let user2 = User{
        name: String::from("Bob"),
        email: String::from("bob@cyt.ir"),
        // we had new values for name and email
        // but wee want to use same values as user1 for is_active and login_count
        is_active: user1.is_active,
        login_count: user1.login_count
    };

    // in another way we can use struct update syntax
    // for example we want to define a new user
    let user3 = User{
        // here we specify the values that we want them to be different
        email: String::from("leon2@cyt.ir"),
        // but we want the rest of values be exactly the values of user1
        // so we use ..
        ..user1 // here we say that the rest of values should be values of user1
    };

    // so if i try to print out the email and the name of user3 what do i get as output!!!?
    // lets see
    println!("here is the email and name of user3: \n email:{} \n name:{}" , user3.email , user3.name);


    // but remember the rules of ownership
    // if its a simple value like bool and int -> the value will be copied
    // so there should be no problem if i try to access login_count of user1
    println!("login count of user1: {}", user1.login_count);
    // but if its a complex value like String, the ownership of that value will be moved
    // so what happend when we said 'name: user1.name' when we defined user3(i know we did not exactly say that, but '..user1' is saying that for us)
    // what happens if i try to access the name of user1 here!!?
    // println!("the name of user1: {}" , user1.name); // i will get an error saying that the value is moved!!!!
    // and the new owner of that value is user3.name
    println!("the name of user3: {}",user3.name);


}

struct User{
    name: String,
    email: String,
    is_active: bool,
    login_count: u32
}
