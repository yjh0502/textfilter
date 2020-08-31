#!/usr/bin/env bash

set -ex

wasm-pack build -t nodejs

sed -i 's/"name": "textfilter",/"name": "@jihyun.yu\/textfilter",/' pkg/package.json
(cd pkg && npm publish --access=public)
