#!/bin/bash
set -e

# Install dependencies using vcpkg
# We build boost-asio and openssl for Android arm64
vcpkg install boost-asio openssl --triplet arm64-android

# Clone i2pd and build using vcpkg toolchain
git clone https://github.com/PurpleI2P/i2pd.git /tmp/i2pd
cd /tmp/i2pd
mkdir build_android && cd build_android

cmake .. \
    -DCMAKE_TOOLCHAIN_FILE=/opt/vcpkg/scripts/buildsystems/vcpkg.cmake \
    -DVCPKG_TARGET_TRIPLET=arm64-android \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-21 \
    -DCMAKE_BUILD_TYPE=Release \
    -DI2PD_WITH_BINARY=ON \
    -DI2PD_WITH_LIBRARY=ON

make -j$(nproc)

# Copy the binary and static libs to workspace
# The workspace is mounted at /workspace in the docker run command
mkdir -p /workspace/crates/iii-i2p/prebuilt/aarch64-linux-android/
cp i2pd /workspace/crates/iii-i2p/prebuilt/aarch64-linux-android/i2pd
cp libi2pd.a /workspace/crates/iii-i2p/
cp libi2pdclient.a /workspace/crates/iii-i2p/
