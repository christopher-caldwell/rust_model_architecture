















fn main() {
    
    
    

    let name = String::from("Alice");
    

    let other_name = name;
    
    

    
    

    
    println!("{}", other_name); 

    
    

    
    
    
    
    
    
    
    

    let greeting = String::from("Hello");

    
    
    print_message(&greeting);

    
    println!("Still have it: {}", greeting);

    
    
    
    
    
    
    
    

    let mut counter = 0;

    add_one(&mut counter);
    

    add_one(&mut counter);
    

    println!("Counter: {}", counter); 

    
    
    
    
    
    
    

    let original = String::from("Bob");
    let copy = original.clone();
    
    

    println!("Original: {}, Copy: {}", original, copy);

    
    
    
    
    
    
    

    let x: u8 = 42;
    let y = x;
    
    println!("x: {}, y: {}", x, y);

    
    
    
}



fn print_message(msg: &str) {
    println!("{}", msg);
    
    
}



fn add_one(value: &mut i32) {
    *value += 1;
    
    
}















