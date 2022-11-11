#!/usr/bin/env bash

rm -rf .vercel/output || echo ""
mkdir .vercel/output

cargo build --release --target=x86_64-unknown-linux-musl
cp target/release/rss-feed .vercel/output/rss-feed
cp ./config.json .vercel/output/config.json || echo ""