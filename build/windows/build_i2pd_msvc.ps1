git clone https://github.com/PurpleI2P/i2pd.git C:\i2pd
cd C:\i2pd
.\build\msvc\build_static.bat
Copy-Item "C:\i2pd\lib\Release\i2pd.lib" "..\..\crates\iii-i2p\"
