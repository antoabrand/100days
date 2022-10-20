fn main() {
    println!("Hello, variables tutorial!");

    //about immutability 
    let x = 5; 
    println!("The value of x is {}", x);

    // x = 6; //compilation error
    println!("The value of x cannot be changed since it is assigned as immutable by default.");


    //proper way to use mutabliity 
    let mut y = 5; 
    println!("The value of y is {}", y);

    y = 1; 
    println!("The value of y is {}", y);

    //shadowing 
    let z =  5; 
    println!("Value of first z is {}", z);
    let z = z + 1; 

    {
        let z = z * 2; 
        println!("Value of inner z is {}", z);
    }
    println!("Value of z + 1 is {}", z);



}
