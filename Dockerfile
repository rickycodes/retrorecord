FROM debian:stretch

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        git \
        gcc \
        libc-dev \
        gcc-arm-linux-gnueabihf \
        libc6-dev-armhf-cross \
        build-essential \
        debhelper \
        dh-systemd \
        wget \
        python \
        openssh-client && \
    rm -rf /var/lib/apt/lists/* && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly && \
    export PATH="/root/.cargo/bin:$PATH" && \
    rustup target add armv7-unknown-linux-gnueabihf

RUN git clone https://github.com/rickycodes/retrorecord.git /home/retrorecord/

RUN mkdir -p /tmp && \
    cd /tmp && \
    wget https://www.openssl.org/source/old/1.1.0/openssl-1.1.0l.tar.gz && \
    tar xzf openssl-1.1.0l.tar.gz && \
    cd openssl-1.1.0l && \
    MACHINE=armv7 ARCH=arm CC=arm-linux-gnueabihf-gcc ./config shared && \
    MACHINE=armv7 ARCH=arm CC=arm-linux-gnueabihf-gcc make

ENV PATH="/root/.cargo/bin:$PATH" \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
    CC_ARMV7_UNKNOWN_LINUX_GNUEABIHF=arm-linux-gnueabihf-gcc \
    ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR=/tmp/openssl-1.1.0l \
    ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_INCLUDE_DIR=/tmp/openssl-1.1.0l/include \
    TARGET_CC=arm-linux-gnueabihf-gcc \
    TARGET_CXX=arm-linux-gnueabihf-g++ \
    OPENSSL_DIR=/tmp/openssl-1.1.0l

WORKDIR /home/retrorecord/

EXPOSE 8000
CMD [ "python", "-m", "SimpleHTTPServer" ]
