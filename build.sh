#!/bin/bash

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  CC_ARMV7_UNKNOWN_LINUX_GNUEABIHF=arm-linux-gnueabihf-gcc \
  ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR=~/openssl/openssl-1.1.1k \
  ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_INCLUDE_DIR=~/openssl/openssl-1.1.1k/include \
  TARGET_CC=arm-linux-gnueabihf-gcc \
  TARGET_CXX=arm-linux-gnueabihf-g++ \
  OPENSSL_DIR=~/openssl/openssl-1.1.1k

cargo build --target=armv7-unknown-linux-gnueabihf --release
