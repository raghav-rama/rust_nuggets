use error_chain::error_chain;
use std::{fs::File, io::copy};
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("image_download").tempdir()?;
    let target = String::from("https://www.rust-lang.org/logos/rust-logo-512x512.png");
    let response = reqwest::get(&target).await?;
    let mut destination = {
        let file_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("File to download: '{}'", file_name);
        let file_name = tmp_dir.path().join(file_name);
        println!("Located at: '{:?}'", file_name);
        File::create(file_name)?
    };
    let tmp_dir = tmp_dir.into_path();
    println!("Temp directory: '{:?}'", tmp_dir);
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut destination)?;
    Ok(())
}
