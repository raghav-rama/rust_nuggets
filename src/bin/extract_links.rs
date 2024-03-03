use error_chain::error_chain;
use select::{document::Document, predicate::Name};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    // let document = Document::from(res.as_str());
    // for node in document.find(Name("a")) {
    //     let link = node.attr("href").unwrap_or("None");
    //     println!("{}", link);
    // }
    Ok(())
}
