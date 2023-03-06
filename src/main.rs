use dotenv::dotenv;


fn main() {
    dotenv().ok();
    let smite_api_key = std::env::var("SMITE_API_KEY").expect("SMITE_API_KEY not found.");
    if smite_api_key.chars().count() == 0 {
        panic!("Value for key SMITE_API_KEY is empty.");
    }
    
}
