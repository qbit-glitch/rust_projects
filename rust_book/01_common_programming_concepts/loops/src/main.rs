
fn main() {
    return_from_loops();
    loop_labels();
    loops_for_collection()
}

/// ## Returning from Loops
fn return_from_loops(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

/// ## Loop Labels to Disambiguate Between Multiple Loops
fn loop_labels(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        loop {
        
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

/// ## Loops through a collection with `for`
fn loops_for_collection(){
    let a = [10,20,30,40,50];
    let mut index = 0;
    println!("------- Using `while` loop --------");
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("------- Using `for` loop --------");
    for element in a {
        println!("the value is: {element}");
    }

    for i in (1..5).rev(){
        println!("{i}");
    }
    println!("LIFTOFFF")
}
