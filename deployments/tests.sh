#!/usr/bin/env bash

# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    if [[ $TARGET = mips-unknown-linux-uclibc ]]; then
        xargo build --target=mips-unknown-linux-uclibc --features mips --release --bin $CRATE_NAME
    else
        cross build --target $TARGET
        cross build --target $TARGET --release

        if [ ! -z $DISABLE_TESTS ]; then
            return
        fi

        cross test --target $TARGET
        cross test --target $TARGET --release
    fi

}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi