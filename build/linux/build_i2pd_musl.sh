#!/bin/bash
set -euxo pipefail

git clone https://github.com/PurpleI2P/i2pd.git /tmp/i2pd
cd /tmp/i2pd
make static musl
cp libi2pd.a /workspace/crates/iii-i2p/
cp libi2pdclient.a /workspace/crates/iii-i2p/
