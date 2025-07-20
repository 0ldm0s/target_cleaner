use std::path::PathBuf;
use std::io::{self, Write};
use anyhow::{Context, Result};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};

/// 清理Rust项目target目录的工具
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root_dir = parse_args(&args).context("解析命令行参数失败")?;
    
    let targets = find_target_dirs(&root_dir)?;
    if targets.is_empty() {
        println!("未找到任何target目录");
        return Ok(());
    }

    confirm_and_delete(targets)?;
    Ok(())
}

/// 解析命令行参数，返回目标目录
fn parse_args(args: &[String]) -> Result<PathBuf> {
    if args.len() != 2 {
        println!("用法: {} <目录路径>", args[0]);
        println!("示例: {} .", args[0]);
        std::process::exit(0);
    }
    Ok(PathBuf::from(&args[1]))
}

/// 查找所有target目录
fn find_target_dirs(root: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();
    let mut visited_dirs = std::collections::HashSet::new();

    // 初始化进度条
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner} 扫描中: {msg}")?);

    for entry in WalkDir::new(root)
        .min_depth(1)
        .max_depth(10)
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            if path.is_dir() {
                let parent = path.parent().unwrap();
                if visited_dirs.contains(parent) {
                    return false;
                }
                
                if path.ends_with("target") {
                    let is_rust_project = parent.join("Cargo.toml").exists();
                    if is_rust_project {
                        visited_dirs.insert(parent.to_path_buf());
                        return true;
                    }
                }
            }
            true
        })
    {
        let entry = entry?;
        pb.set_message(entry.path().display().to_string());
        pb.tick();

        if entry.file_name() == "target" && entry.path().is_dir() {
            targets.push(entry.path().to_path_buf());
        }
    }

    pb.finish_with_message("扫描完成");
    Ok(targets)
}

/// 交互式确认并删除目录
fn confirm_and_delete(targets: Vec<PathBuf>) -> Result<()> {
    println!("找到以下target目录:");
    for (i, path) in targets.iter().enumerate() {
        println!("{}. {}", i + 1, path.display());
    }

    print!("是否删除这些目录？(y/N): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().eq_ignore_ascii_case("y") {
        for path in targets {
            std::fs::remove_dir_all(&path)
                .with_context(|| format!("删除目录失败: {}", path.display()))?;
            println!("已删除: {}", path.display());
        }
    } else {
        println!("取消删除操作");
    }

    Ok(())
}
