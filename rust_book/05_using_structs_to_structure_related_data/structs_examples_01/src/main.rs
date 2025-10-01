#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main(){
    let mut user1 = User{
        active: true,
        username: String::from("qbit_glitch"),
        email: String::from("qbit_glitch@gmail.com"),
        sign_in_count:10,
    };
    user1.email = String::from("qbit_ai@ai.com");
    println!("{:?}", user1);
}

