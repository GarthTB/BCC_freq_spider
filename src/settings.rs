use std::io;
use std::path::Path;

fn get_filepath(prompt: &str) -> String {
    println!("{prompt}");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        let filepath = input.trim().to_string();
        if Path::new(&filepath).exists() {
            return filepath;
        } else {
            println!("文件不存在，请重新输入！");
        }
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    println!("{prompt}");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("输入无效！请重新输入！"),
        }
    }
}

pub fn get_settings() -> (String, usize, u64) {
    let words_path = get_filepath("请输入一个文本文件路径，其中每行为一个待爬取的项：");
    let mut concurrency = get_input::<usize>("请输入并发数：");
    if concurrency == 0 {
        println!("已使用默认并发数：8");
        concurrency = 8;
    }
    let mut timeout_s = get_input::<u64>("请输入网络超时秒数：");
    if timeout_s == 0 {
        println!("已使用默认超时秒数：30");
        timeout_s = 30;
    }
    (words_path, concurrency, timeout_s)
}
