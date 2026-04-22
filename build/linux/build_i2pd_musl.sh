#!/bin/bash
set -euxo pipefail

ROOT_DIR=$(pwd)
git clone --depth 1 https://github.com/PurpleI2P/i2pd.git /tmp/i2pd
cd /tmp/i2pd
mkdir -p build_linux && cd build_linux

cmake ../build \
    -DCMAKE_BUILD_TYPE=Release \
    -DWITH_UPNP=OFF \
    -DWITH_AESNI=ON \
    -DBUILD_STATIC=ON \
    -DBUILD_SHARED=OFF \
    -DWITH_LIBRARY=ON

make -j$(nproc)
cp libi2pd.a libi2pdclient.a "$ROOT_DIR/crates/iii-i2p/"
