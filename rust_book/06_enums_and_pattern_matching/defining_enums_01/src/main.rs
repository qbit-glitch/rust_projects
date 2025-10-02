/**
 * NOTE: This code hasn't been compiled and it contains a bunch of warning and 1 small error message
 */

enum IpAddrKind{
    v4,
    v6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind){
    // todo
}


/**
 * Enums can Hold different kinds of data associated with it
*/
enum Message{
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor (i32,i32,i32),
}


// Enums can have their own associated methods defined using `impl` 
impl Message{
    fn call(&self) {
        // method body would be defined here
    }
}

// Most used enum -> Option<T>
enum Option<T>{
    Some(T),
    None,
}





fn main() {
    println!("Hello, Enums");
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

    let home = IpAddr{
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("Hello! from Enum which can hold different kinds of messages ..."))

    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i8> = None;

}

