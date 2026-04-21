






















use std::sync::Arc;
use std::thread;

fn main() {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    

    
    
    let shared_message = Arc::new(String::from("Hello from shared data!"));
    
    
    

    
    
    
    let message_for_thread_1 = Arc::clone(&shared_message);
    
    
    
    
    

    
    let message_for_thread_2 = Arc::clone(&shared_message);

    
    let handle_1 = thread::spawn(move || {
        
        
        
        
        
        println!("Thread 1 says: {}", message_for_thread_1);
        
        
    });

    
    let handle_2 = thread::spawn(move || {
        println!("Thread 2 says: {}", message_for_thread_2);
        
    });

    
    handle_1.join().unwrap();
    handle_2.join().unwrap();

    
    println!("Main thread says: {}", shared_message);

    
    

    
    
    

    let expensive_data = Arc::new(vec![1, 2, 3, 4, 5]);

    
    
    let handle_a = Arc::clone(&expensive_data);

    
    
    let separate_copy = (*expensive_data).clone();

    
    
    println!("Same data: {:?}", handle_a);
    println!("Independent copy: {:?}", separate_copy);

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}


























