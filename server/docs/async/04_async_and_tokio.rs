








































async fn get_user_name() -> String {
    
    
    
    
    
    
    

    
    let name = tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    
    

    String::from("Alice")
}





async fn example_lazy() {
    
    
    let _future = get_user_name();
    

    
    let name = get_user_name().await;
    println!("Got: {}", name);
}

















#[tokio::main]
async fn main() {
    
    
    
    
    
    
    
    
    
    
    

    let name = get_user_name().await;
    println!("Hello, {}", name);

    
    
    
    
    
    
    
    
    

    let (result_a, result_b) = tokio::join!(
        fetch_from_database("users"),
        fetch_from_database("orders"),
    );
    
    
    println!("A: {}, B: {}", result_a, result_b);

    
    
    
    
    
    
    
    
    
    

    let handle = tokio::spawn(async {
        fetch_from_database("products").await
    });

    
    println!("Doing other work...");

    
    let result = handle.await.unwrap();
    println!("Spawned task got: {}", result);
}

async fn fetch_from_database(table: &str) -> String {
    
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    format!("data from {}", table)
}







































































