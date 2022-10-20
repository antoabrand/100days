fn main() {
    println!("Hello, world!");

    another_fn();
    statements_only();
    println!("{}", expression_fn());
}

fn another_fn(){
    println!("Hello, from another_fn!");
}


fn statements_only(){
    println!("In a statment only.");
}

fn expression_fn() -> String {
    //when returning values, no need to use a return statement.  Also, do not place a ; at end otherwise it is compiled as a statement and leads to weird side effects so it's not allowed
    "In an expression".to_string()
}