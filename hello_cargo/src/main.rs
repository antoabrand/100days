#![allow(dead_code)]


#[derive(Debug)]
struct Person {
    name: String, 
    age: u8
}
fn main() {
    let xs : [i32; 5] = [1,2,3,4,5];
    println!("Array xs first element: {}", xs[0]);

    let yx : [i32; 500] = [0;500];
    analyze_slice(&xs);
    analyze_slice(&yx[..]);
}

fn analyze_slice(slice : &[i32]){
    println!("The first element of the slice is : {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}