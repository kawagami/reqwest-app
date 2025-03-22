use reqwest;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://holodex.net/";
    let body = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&body);

    // 1. 先找到 id="bodywrap" 的 div
    let bodywrap_selector = Selector::parse("div#bodywrap").unwrap();
    if let Some(bodywrap) = document.select(&bodywrap_selector).next() {
        // 2. 在 bodywrap 內搜尋 img
        let img_selector = Selector::parse("img").unwrap();
        for img in bodywrap.select(&img_selector) {
            let src = img.attr("src").unwrap_or("No src found");
            let alt = img.attr("alt").unwrap_or("No alt found");
            println!("Image Src: {}, Alt: {}", src, alt);
        }
    }

    Ok(())
}
