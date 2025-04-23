@echo off

cd .\backend\
cargo clean
cd ..
pnpm run build:windows

.\backend\target\debug\qwikpage.exe
