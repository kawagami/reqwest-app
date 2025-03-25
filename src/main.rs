mod structs;

use reqwest_app::WebScraper;
use std::collections::HashMap;
use structs::stocks::StockBuyback;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let scraper = WebScraper::new();

    // 創建表單參數
    let mut params = HashMap::new();
    params.insert("step".to_string(), "1".to_string());
    params.insert("firstin".to_string(), "1".to_string());
    params.insert("off".to_string(), "1".to_string());
    params.insert("TYPEK".to_string(), "sii".to_string());
    params.insert("d1".to_string(), "1140101".to_string());
    params.insert("d2".to_string(), "1140331".to_string());
    params.insert("RD".to_string(), "1".to_string());

    // 使用基本 POST 方法
    let html_content = scraper
        .post_form("https://mopsov.twse.com.tw/mops/web/ajax_t35sc09", params)
        .await?;

    // // 如果需要添加特定的請求頭
    // let mut headers = HashMap::new();
    // headers.insert(
    //     "Content-Type".to_string(),
    //     "application/x-www-form-urlencoded".to_string(),
    // );

    // 解析股票回購資訊
    let buybacks = StockBuyback::parse_from_html(&html_content);

    // 打印每一筆回購資訊
    for buyback in &buybacks {
        println!(
            "公司：{} ({}), 決議日期：{}",
            buyback.公司名稱, buyback.公司代號, buyback.董事會決議日期
        );
    }

    // 打印統計資訊
    println!("\n{}", StockBuyback::get_total_stats(&buybacks));

    Ok(())
}
