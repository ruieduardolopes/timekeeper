#!/usr/bin/env bash

set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # Builds for iOS are done on OSX, but require the specific target to be
    # installed.
    case $TARGET in
        aarch64-apple-ios)
            rustup target install aarch64-apple-ios
            ;;
        armv7-apple-ios)
            rustup target install armv7-apple-ios
            ;;
        armv7s-apple-ios)
            rustup target install armv7s-apple-ios
            ;;
        i386-apple-ios)
            rustup target install i386-apple-ios
            ;;
        x86_64-apple-ios)
            rustup target install x86_64-apple-ios
            ;;
    esac

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/rust-embedded/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-embedded/cross \
           --tag $tag \
           --target $target

    echo -e "Downloading OpenWRT SDK from OpenWRT archive..."

    curl -LSfs https://archive.openwrt.org/barrier_breaker/14.07/ar71xx/mikrotik/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2.tar.bz2 > OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2.tar.bz2
    tar -xf OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2.tar.bz2

    export SDK="$(pwd)/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2"
    echo -e "Set SDK variable as: $SDK"
    export STAGING_DIR="$(pwd)/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2/staging_dir"
    echo -e "Set STAGING_DIR variable as: $STAGING_DIR"
    export PATH="$(pwd)/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2/staging_dir/toolchain-mips_34kc_gcc-4.8-linaro_uClibc-0.9.33.2/bin:$PATH"
    echo -e "Set PATH variable as: $PATH"

    echo -e "Copying cargo config to ~/.cargo"
    cp deployments/config ~/.cargo/

    echo -e "Installing xargo and adding component rust-src"
    cargo install --force xargo
    rustup component add rust-src
}

main