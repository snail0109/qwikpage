#!/bin/bash

# 执行 pnpm build:admin 命令
echo "开始构建 admin..."
pnpm run build:admin

# 检查构建是否成功
if [ $? -ne 0 ]; then
    echo "构建失败，退出脚本"
    exit 1
fi

# 设置源目录和目标目录
SOURCE_DIR="dist/admin"
TARGET_DIR="src-tauri/assets/admin"

# 确保目标目录存在
mkdir -p "$TARGET_DIR"

# 复制文件，如果目标已存在则覆盖
echo "正在将 $SOURCE_DIR 复制到 $TARGET_DIR..."
cp -rf "$SOURCE_DIR"/* "$TARGET_DIR"

# 检查复制是否成功
if [ $? -eq 0 ]; then
    echo "复制完成！"
else
    echo "复制过程中发生错误"
    exit 1
fi