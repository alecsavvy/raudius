use client::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // cargo run --example stream D7KyD
    let track_id = std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .pop()
        .unwrap_or_else(|| "D7KyD".to_string());

    println!("string track {}", track_id);
    Ok(())
}
