// main.rs
use reqwest_app::{DataExtractor, HtmlDocument, WebScraper};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化爬蟲
    let scraper = WebScraper::new();
    let url = "https://kawa.homes";

    // 獲取頁面內容
    let html = scraper.fetch_page(url).await?;

    // 解析 HTML
    let document = HtmlDocument::new(&html);

    // 創建數據提取器
    let extractor = DataExtractor::new(&document);

    // 提取並顯示鏈接信息
    println!("=== 鏈接信息 ===");
    match extractor.extract_attribute_pairs("main", "a", "href", "title") {
        Ok(links) => {
            if links.is_empty() {
                println!("沒有找到鏈接");
            } else {
                for (href, title) in links {
                    println!("Link: {}, Title: {}", href, title);
                }
            }
        }
        Err(e) => println!("提取鏈接時出錯: {:?}", e),
    }

    // 提取並顯示圖片信息
    println!("\n=== 圖片信息 ===");
    match extractor.extract_attribute_pairs("main", "img", "src", "alt") {
        Ok(images) => {
            if images.is_empty() {
                println!("沒有找到圖片");
            } else {
                for (src, alt) in images {
                    println!("Image Src: {}, Alt: {}", src, alt);
                }
            }
        }
        Err(e) => println!("提取圖片時出錯: {:?}", e),
    }

    Ok(())
}
