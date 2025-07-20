# Rust Target Cleaner

一个用于清理Rust项目target目录的命令行工具，采用GPL-3.0许可证。

## 功能特性

- 递归查找指定目录下的所有Rust项目target目录
- 智能跳过已处理的同级目录
- 交互式确认删除操作
- 安全删除验证

## 安装

```bash
cargo install --path .
```

## 使用方法

```bash
target_cleaner <目录路径>
```

示例:
```bash
target_cleaner .
```

## 许可证

```text
Copyright (C) 2025 Your Name

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## 贡献

欢迎提交Issue和Pull Request。