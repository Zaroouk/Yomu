use std::fs::File;
use std::io::copy;


#[tokio::main]
async fn download_image(data:Chapter,url:&str) -> Result<(), Box<dyn std::error::Error>>{
    let mut response = reqwest::blocking::get(url)?;
    let bytes = reqwest::get(url).await?.bytes().await?;

    fs::write("image.jpg",bytes).await?;

    println!("Image Downloaded!");

    Ok(())
}