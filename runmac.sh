#!/bin/bash
pnpm install

pnpm run build:mac

./backend/target/debug/bundle/macos/QwikPage.app/Contents/MacOS/qwikpage
