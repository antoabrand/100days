fn main() {
    println!("Hello, conditions!");
    conditions();
    loops();
}

fn conditions(){

    let x = true; 

    let y = if x {5} else {6};

    println!("The value of y is: {}", y);
}

fn loops(){

    let mut count : i32 = 0; 
    'my_loop : loop {
        println!("Count is at: {}", count);

        if count == 10 {
            println!("got to end of my loop"); 
            break
        }

        count += 1; 
    }
}