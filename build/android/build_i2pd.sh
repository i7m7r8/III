#!/bin/bash
set -e

# Build i2pd for Android using CMake with NDK
git clone --depth 1 https://github.com/PurpleI2P/i2pd.git /tmp/i2pd
cd /tmp/i2pd
mkdir -p build_android && cd build_android

cmake ../build \
    -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_HOME/build/cmake/android.toolchain.cmake \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-21 \
    -DCMAKE_BUILD_TYPE=Release \
    -DWITH_UPNP=OFF \
    -DBUILD_STATIC=ON \
    -DWITH_LIBRARY=ON

make -j$(nproc)
mkdir -p /workspace/crates/iii-i2p/prebuilt/aarch64-linux-android/
cp libi2pd.a libi2pdclient.a /workspace/crates/iii-i2p/
cp i2pd /workspace/crates/iii-i2p/prebuilt/aarch64-linux-android/i2pd 2>/dev/null || true
