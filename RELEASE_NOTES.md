# v0.1.0 Release Notes

## 功能特性
- 实现Rust项目target目录清理工具
- 支持智能跳过已处理的同级目录
- 交互式确认删除操作
- 多平台支持(Windows/Linux/macOS)
- 进度条显示扫描过程

## 使用说明
```bash
target_cleaner <目录路径>
```

## Git发布命令
```bash
# 添加所有更改
git add .

# 提交更改
git commit -m "准备v0.1.0发布"

# 创建tag
git tag -a v0.1.0 -m "v0.1.0版本发布"

# 推送tag触发自动构建
git push origin v0.1.0
```

## 已知问题
- 暂无

## 未来计划
- 添加更多平台支持
- 优化性能