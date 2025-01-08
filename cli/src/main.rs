use clap::{Parser, Subcommand};
use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author = "Jalever", version = "0.1.0", about = "A simple CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum FileOperation {
    Check,
    Create,
    Delete,
    Append { content: String },
    Read,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Greet {
        name: String,
    },
    Add {
        x: i32,
        y: i32,
    },
    File {
        #[arg(short, long)]
        path: String,
        #[command(subcommand)]
        operation: FileOperation,
    },
}

fn handle_file_operation(
    path: &str,
    operation: FileOperation,
) -> Result<(), Box<dyn std::error::Error>> {
    match operation {
        FileOperation::Check => {
            if Path::new(path).exists() {
                println!("File exists: {}", path);
            } else {
                println!("File does not exist: {}", path);
            }
        }
        FileOperation::Create => {
            File::create(path)?;
            println!("File created: {}", path);
        }
        FileOperation::Delete => {
            remove_file(path)?;
            println!("File deleted: {}", path);
        }
        FileOperation::Append { content } => {
            let mut file = OpenOptions::new().append(true).open(path)?;
            writeln!(file, "{}", content)?;
            println!("Content appended to file: {}", path);
        }
        FileOperation::Read => {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("File contents: {}", contents);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        // This program greets the user with the provided name.
        // Usage: cargo run -- greet <NAME>
        // Example: cargo run -- greet "Rustacean"
        Commands::Greet { name } => println!("Hello, {}!", name),
        Commands::Add { x, y } => println!("{} + {} = {}", x, y, x + y),
        Commands::File { path, operation } => {
            handle_file_operation(&path, operation)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_file_operations() {
        let test_file = "test_file.txt";

        // 清理测试文件（如果存在）
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).unwrap();
        }

        // 测试文件创建
        handle_file_operation(test_file, FileOperation::Create).unwrap();
        assert!(Path::new(test_file).exists(), "文件创建失败");

        // 测试文件内容追加
        handle_file_operation(
            test_file,
            FileOperation::Append {
                content: "Hello, Rust!".to_string(),
            },
        )
        .unwrap();
        let mut contents = String::new();
        File::open(test_file)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!(contents, "Hello, Rust!\n", "文件内容追加失败");

        // 测试文件读取
        handle_file_operation(test_file, FileOperation::Read).unwrap();

        // 测试文件删除
        handle_file_operation(test_file, FileOperation::Delete).unwrap();
        assert!(!Path::new(test_file).exists(), "文件删除失败");
    }
}
