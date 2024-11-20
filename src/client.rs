use hyper::{Client, Uri};
use hyper::client::HttpConnector;
use hyper::body::HttpBody; // 導入 HttpBody 特徵
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 建立 HTTP 客戶端
    let client: Client<HttpConnector> = Client::builder().build(HttpConnector::new());

    // 定義目標 URI
    //let uri = "http://example.com".parse::<Uri>()?;
    let uri = "http://localhost:3000".parse::<Uri>()?;

    // 發送 GET 請求
    let mut resp = client.get(uri).await?;

    // 輸出狀態碼
    println!("Response: {}", resp.status());

    // 逐塊讀取回應的 body
    while let Some(chunk) = resp.body_mut().data().await {
        let chunk = chunk?; // 確保該 chunk 無錯誤
        stdout().write_all(chunk.as_ref()).await?; // 使用 as_ref() 轉換為 &[u8]
    }

    Ok(())
}

