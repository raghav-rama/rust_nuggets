#[cfg(test)]
mod image_download_test {
    use error_chain::error_chain;
    use std::{fs::File, io::copy};
    use tempfile::Builder;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpClient(reqwest::Error);
        }
    }

    #[tokio::test]
    async fn id_downloads_the_image() -> Result<()> {
        let tempdir = Builder::new().prefix("image").tempdir()?;
        let image_url = String::from("https://www.rust-lang.org/logos/rust-logo-512x512.png");
        let response = reqwest::get(&image_url).await?;
        let mut destination = {
            let file_name = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");
            let file_name = tempdir.path().join(file_name);
            println!("File to download: '{}'", file_name.display());
            File::create(file_name)?
        };
        let tempdir = tempdir.into_path();
        println!("Temp directory: '{:?}'", tempdir);
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut destination)?;
        assert_eq!(tempdir.exists(), true);
        assert_eq!(tempdir.is_dir(), true);
        assert_eq!(tempdir.read_dir()?.count(), 1);
        assert_eq!(destination.metadata()?.is_file(), true);
        Ok(())
    }
}
