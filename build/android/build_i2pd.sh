#!/bin/bash
set -e
git clone https://github.com/PurpleI2P/i2pd.git /tmp/i2pd
cd /tmp/i2pd
export CC=aarch64-linux-android21-clang
export CXX=aarch64-linux-android21-clang++
make static
cp libi2pd.a /workspace/crates/iii-i2p/
cp libi2pdclient.a /workspace/crates/iii-i2p/
