use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockBuyback {
    pub 序號: String,
    pub 公司代號: String,
    pub 公司名稱: String,
    pub 董事會決議日期: String,
    pub 買回目的: String,
    pub 買回股份總金額上限: String,
    pub 預定買回股數: String,
    pub 買回價格區間_最低: String,
    pub 買回價格區間_最高: String,
    pub 預定買回期間_起: String,
    pub 預定買回期間_迄: String,
    pub 是否執行完畢: String,
    pub 本次已買回股數: String,
    pub 本次執行完畢已註銷或轉讓股數: String,
    pub 本次已買回股數佔預定買回股數比例: String,
    pub 本次已買回總金額: String,
    pub 本次平均每股買回價格: String,
    pub 本次買回股數佔公司已發行股份總數比例: String,
    pub 本次未執行完畢之原因: String,
}

impl StockBuyback {
    pub fn parse_from_html(html_content: &str) -> Vec<Self> {
        let document = Html::parse_document(html_content);

        // 選擇包含股票回購資訊的表格行
        let row_selector =
            Selector::parse("table.hasBorder tr.even, table.hasBorder tr.odd").unwrap();

        document
            .select(&row_selector)
            .filter_map(|row| {
                // 選擇每一行中的所有單元格
                let td_selector = Selector::parse("td").unwrap();
                let cells: Vec<_> = row.select(&td_selector).collect();

                // 確保有足夠的單元格
                if cells.len() >= 16 {
                    Some(StockBuyback {
                        序號: cells[0].inner_html().trim().to_string(),
                        公司代號: cells[1].inner_html().trim().to_string(),
                        公司名稱: cells[2].inner_html().trim().to_string(),
                        董事會決議日期: cells[3].inner_html().trim().to_string(),
                        買回目的: cells[4].inner_html().trim().to_string(),
                        買回股份總金額上限: cells[5].inner_html().trim().to_string(),
                        預定買回股數: cells[6].inner_html().trim().to_string(),
                        買回價格區間_最低: cells[7].inner_html().trim().to_string(),
                        買回價格區間_最高: cells[8].inner_html().trim().to_string(),
                        預定買回期間_起: cells[9].inner_html().trim().to_string(),
                        預定買回期間_迄: cells[10].inner_html().trim().to_string(),
                        是否執行完畢: cells[11].inner_html().trim().to_string(),
                        本次已買回股數: cells[13].inner_html().trim().to_string(),
                        本次執行完畢已註銷或轉讓股數: cells[14]
                            .inner_html()
                            .trim()
                            .to_string(),
                        本次已買回股數佔預定買回股數比例: cells[14]
                            .inner_html()
                            .trim()
                            .to_string(),
                        本次已買回總金額: cells[15].inner_html().trim().to_string(),
                        本次平均每股買回價格: cells[16].inner_html().trim().to_string(),
                        本次買回股數佔公司已發行股份總數比例: cells[17]
                            .inner_html()
                            .trim()
                            .to_string(),
                        本次未執行完畢之原因: cells[18].inner_html().trim().to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    // 額外的統計方法
    pub fn get_total_stats(buybacks: &[Self]) -> String {
        let total_companies = buybacks.len();
        let completed_buybacks = buybacks.iter().filter(|b| b.是否執行完畢 == "Y").count();

        format!(
            "總公司數：{} 家\n已完成回購：{} 家\n完成率：{:.2}%",
            total_companies,
            completed_buybacks,
            (completed_buybacks as f64 / total_companies as f64) * 100.0
        )
    }
}

// 預計完成 取得對照時間的盤後價 兩者間的差異
// 取得 起始 & 結束 時間點
// 取兩個時間點的盤後價
// 計算差異
pub struct ClosingPrice {
    pub stock_symbol: String,             // 股票代碼
    pub name: String,                     // 股票中文名稱
    pub price: f64,                       // 盤後價格
    pub volume: u64,                      // 成交量
    pub timestamp: chrono::NaiveDateTime, // 時間戳
}
