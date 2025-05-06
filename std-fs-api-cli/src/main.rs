use inquire::{Select, Text};
use colored::*;
use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
enum FsOperation {
    ReadToString,
    ReadBinary,
    ReadLines,
    WriteString,
    WriteAppend,
    ReadMetadata,
    ReadDir,
    Exit,
}

fn main() {
    loop {
        let ops = vec![
            ("读取整个文件夹为字符串", FsOperation::ReadToString),
            ("读取文件为二进制", FsOperation::ReadBinary),
            ("逐行读取文件", FsOperation::ReadLines),
            ("写入（覆盖）为字符串", FsOperation::WriteString),
            ("追加写入为字符串", FsOperation::WriteAppend),
            ("读取文件元数据", FsOperation::ReadMetadata),
            ("遍历目录", FsOperation::ReadDir),
            ("退出", FsOperation::Exit),
        ];
        let op_labels: Vec<&str> = ops.iter().map(|(l, _)| *l ).collect();
        let choice = Select::new("请选择操作", op_labels.clone())
            .with_vim_mode(true)
            .prompt();

        let op = match choice {
            Ok(label) => ops.iter().find(|(l, _)| l == &label).unwrap().1,
            Err(_) => {
                println!("{}", "已取消输入，程序退出".yellow().bold());
                break;
            }
        };

        match op {
            FsOperation::ReadToString => {
                let path = ask_path("请输入文件路径");
                match fs::read_to_string(&path) {
                    Ok(contents) => println!("{}\n{}", "读取成功:".green(), contents),
                    Err(e) => println!("{} : {}", "读取失败".red(), e),
                }
            }
            FsOperation::ReadBinary => {
                let path = ask_path("请输入要读取的（二进制）文件路径：");
                match fs::read(&path) {
                    Ok(data) => println!("{}: {} 字节\n{:02X?}", "读取成功".green(), data.len(), &data[..std::cmp::min(32, data.len())]),
                    Err(e) => println!("{} : {}", "读取失败".red(), e),
                }
            }
            FsOperation::ReadLines => {
                let path = ask_path("请输入要逐行读取的文件路径：");
                match File::open(&path) {
                    Ok(file) => {
                        let reader = BufReader::new(file);
                        println!("{}", "文件内容".green());
                        for (i, line) in reader.lines().enumerate() {
                            match line {
                                Ok(l) => println!("{:>3}: {}", i, l),
                                Err(e) => println!("{}: {}", "读取行出错".red(), e)
                            }
                        }
                    },
                    Err(e) => println!("{} : {}", "打开失败".red(), e),
                }
            }
            FsOperation::WriteString => {
                let path = ask_path("请输入要写入的文件路径：");
                let content = ask_any("请输入要写入的内容：");
                match fs::write(&path, &content) {
                    Ok(_) => println!("{}: {}", "写入成功".green(), path),
                    Err(e) => println!("{} : {}", "写入失败".red(), e),
                }
            }
            FsOperation::WriteAppend => {
                let path = ask_path("请输入要追加写入的文件路径：");
                let content = ask_any("请输入要追加写入的内容：");
                let mut file = match OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&path) {
                        Ok(f) => f,
                        Err(e) => {
                            println!("{} : {}", "打开失败".red(), e);
                            continue;
                        }
                    };
                if let Err(e) = writeln!(file, "{}", content) {
                    println!("{} : {}", "写入失败".red(), e);
                } else {
                    println!("{}: {}", "写入成功".green(), path);
                }   
            }
            FsOperation::ReadMetadata => {
                let path = ask_path("请输入要读取元数据的文件路径：");
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        println!("{}", "元数据如下：".green());
                        println!("{:#?}", metadata);
                    },
                    Err(e) => println!("{}: {}", "读取元数据失败".red(), e),
                }
            }
            FsOperation::ReadDir => {
                let path = ask_path("请输入目录路径（遍历目录）：");
                match fs::read_dir(&path) {
                    Ok(entries) => {
                        println!("{}", "目录下包含：".green());
                        for entry in entries {
                            match entry {
                                Ok(e) => {
                                    println!(
                                        "{}",
                                        e.path().display().to_string().blue()
                                    );
                                }
                                Err(e) => println!("{}: {}", "读取目录项失败".red(), e),
                            }
                        }
                    }
                    Err(e) => println!("{}: {}", "遍历目录失败".red(), e),
                }
            }
            FsOperation::Exit => {
                println!("{}", "程序已退出".yellow());
                break;
            }
        }
    }
}

fn ask_path(prompt: &str) -> String {
    loop {
        match Text::new(prompt).prompt() {
            Ok(s) if !s.is_empty() => return s.trim().to_owned(),
            Ok(_) => println!("{}", "路径不可为空".yellow()),
            Err(_) => println!("{}", "已取消输入，程序退出".yellow()),
        }
    }
}

fn ask_any(prompt: &str) -> String {
    match Text::new(prompt).prompt() {
        Ok(s) => s.trim().to_owned(),
        Err(_) => "".to_owned(),
    }
}