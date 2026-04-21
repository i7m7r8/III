FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    git make cmake g++ curl unzip \
    && rm -rf /var/lib/apt/lists/*

ENV NDK_VERSION=r26d
RUN curl -L https://dl.google.com/android/repository/android-ndk-${NDK_VERSION}-linux.zip -o /tmp/ndk.zip \
    && unzip /tmp/ndk.zip -d /opt \
    && rm /tmp/ndk.zip

ENV PATH=/opt/android-ndk-${NDK_VERSION}/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

COPY build_i2pd.sh /build_i2pd.sh
RUN chmod +x /build_i2pd.sh

ENTRYPOINT ["/build_i2pd.sh"]
