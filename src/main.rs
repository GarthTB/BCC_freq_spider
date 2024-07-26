use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::Client;
use tokio::sync::Semaphore;

mod analyser;
mod settings;

#[tokio::main]
async fn main() {
    // 用户输入配置
    let (words_path, concurrency, timeout_s) = settings::get_settings();

    // 创建客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_s))
        .build()
        .expect("创建客户端失败");

    // 读取词语文件
    let file = File::open(&words_path).expect("读取文件失败");
    let lines = io::BufReader::new(file).lines();
    let words: Vec<String> = lines.filter_map(|l| l.ok()).collect();

    // 控制并发
    let semaphore = Arc::new(Semaphore::new(concurrency));

    // 使用Mutex来在异步环境中共享写入状态
    let mut path = PathBuf::from(words_path);
    path.set_file_name("爬取结果.txt");
    let result_file = File::create(path).expect("创建结果文件失败");
    let result_file = Arc::new(Mutex::new(result_file));

    // 爬取每个词语
    let mut handles = vec![];
    for word in words {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("获取信号量失败");
        let client = client.clone();
        let result_file = result_file.clone();
        let handle = tokio::spawn(async move {
            let count = analyser::get_count(&client, &word).await;
            let mut file = result_file.lock().expect("写入文件锁定失败");
            match count {
                Ok(count) => writeln!(file, "{word}\t{count}").expect("写入文件失败"),
                Err(e) => {
                    println!("{word}的词频：-1，因为{e}");
                    writeln!(file, "{word}\t-1").expect("写入文件失败");
                }
            }
            drop(permit); // 释放信号量
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await.expect("任务失败");
    }
}
