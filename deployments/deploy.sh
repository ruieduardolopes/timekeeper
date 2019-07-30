#!/usr/bin/env bash
set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    export PATH="$(pwd)/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2/staging_dir/toolchain-mips_34kc_gcc-4.8-linaro_uClibc-0.9.33.2/bin:$PATH"
    echo -e "Set PATH variable as: $PATH"

    if [ $TARGET = mips-unknown-linux-uclibc ]; then
        echo -e "Building with xargo..."
        xargo build --target=mips-unknown-linux-uclibc --features mips --release --bin $CRATE_NAME

        cp target/$TARGET/release/$CRATE_NAME $stage/

        cd $stage
        tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
        cd $src

        rm -rf $stage
    else
        echo -e "Building with cargo-cross..."
        # TODO Update this to build the artifacts that matter to you
        cross rustc --bin timekeeper --target $TARGET --release -- -C lto

        # TODO Update this to package the right artifacts
        cp target/$TARGET/release/timekeeper $stage/

        cd $stage
        tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
        cd $src

        rm -rf $stage
    fi

}

main