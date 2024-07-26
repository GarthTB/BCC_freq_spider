use regex::Regex;
use reqwest::Client;

pub async fn get_count(client: &Client, word: &str) -> Result<String, reqwest::Error> {
    let text = client
        .get(&format!("https://bcc.blcu.edu.cn/zh/search/0/{word}"))
        .send()
        .await?
        .text()
        .await?;

    if text.contains(&format!("\"input\" value=\"{word}\"")) {
        let re = Regex::new(r#""totalnum" value="(\d+)"#).unwrap();
        if let Some(caps) = re.captures(&text) {
            if let Some(matched) = caps.get(1) {
                let keyword = matched.as_str().to_string();
                println!("{word}的词频：{keyword}");
                return Ok(keyword);
            }
        }
        println!("{word}的词频：-1，网站中没有该词的信息");
        Ok("-1".to_string())
    } else {
        println!("{word}的词频：-1，页面错误或有网站不支持的字符");
        Ok("-1".to_string())
    }
}
