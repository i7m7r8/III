FROM ubuntu:22.04

RUN apt-get update && apt-get install -y     git make cmake g++ curl unzip zip tar pkg-config ninja-build python3     && rm -rf /var/lib/apt/lists/*

# Install vcpkg
RUN git clone https://github.com/microsoft/vcpkg.git /opt/vcpkg &&     /opt/vcpkg/bootstrap-vcpkg.sh

# Install Android NDK r26d
RUN curl -L https://dl.google.com/android/repository/android-ndk-r26d-linux.zip -o /tmp/ndk.zip &&     unzip /tmp/ndk.zip -d /opt &&     rm /tmp/ndk.zip

ENV ANDROID_NDK_HOME=/opt/android-ndk-r26d
ENV PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

COPY build_i2pd.sh /build_i2pd.sh
RUN chmod +x /build_i2pd.sh

ENTRYPOINT ["/build_i2pd.sh"]
