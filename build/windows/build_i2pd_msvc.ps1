# Store the root directory
$ROOT_DIR = Get-Location

# Build i2pd using CMake and vcpkg
git clone https://github.com/PurpleI2P/i2pd.git C:\i2pd
cd C:\i2pd
New-Item -ItemType Directory -Path build_win
cd build_win

cmake ../build `
    -DCMAKE_TOOLCHAIN_FILE="$env:VCPKG_INSTALLATION_ROOT/scripts/buildsystems/vcpkg.cmake" `
    -DCMAKE_BUILD_TYPE=Release `
    -DBUILD_STATIC=ON `
    -DBUILD_SHARED=OFF `
    -DI2PD_WITH_BINARY=ON

cmake --build . --config Release -j $env:NUMBER_OF_PROCESSORS

# Copy artifacts back to the workspace
New-Item -ItemType Directory -Force -Path "$ROOT_DIR\crates\iii-i2p\"
Copy-Item "Release\i2pd.exe" "$ROOT_DIR\crates\iii-i2p\i2pd.exe"
Copy-Item "Release\libi2pd.lib" "$ROOT_DIR\crates\iii-i2p\"
