use reqwest::Client;

pub async fn get_count(client: &Client, word: &str) -> Result<u64, reqwest::Error> {
    let text = client
        .get(&format!("https://bcc.blcu.edu.cn/zh/search/0/{word}"))
        .send()
        .await?
        .text()
        .await?;
    let parts: Vec<&str> = text.split("条目数量：").collect();
    if parts.len() > 1 {
        parts[1]
            .split_whitespace()
            .next()
            .unwrap_or("0")
            .parse()
            .ok()
    } else {
        Ok(0)
    }
}
