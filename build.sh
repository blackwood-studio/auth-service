#!/bin/bash

WINDOWS="x86_64-pc-windows-gnu"
LINUX="x86_64-unknown-linux-gnu"

WINDOWS_DESTINATION="auth-service-$WINDOWS"
LINUX_DESTINATION="auth-service-$LINUX"

rm -fr ./target/release
rm -fr ./target/$WINDOWS
rm -fr ./target/$LINUX

rm -fr ./target/$WINDOWS_DESTINATION
rm -fr ./target/$LINUX_DESTINATION

rustup target add $WINDOWS
rustup target add $LINUX

cargo build --release --target $WINDOWS
cargo build --release --target $LINUX

mkdir ./target/$WINDOWS_DESTINATION
mkdir ./target/$LINUX_DESTINATION

cp -R ./target/$WINDOWS/release/. ./target/$WINDOWS_DESTINATION
cp -R ./target/$LINUX/release/. ./target/$LINUX_DESTINATION

cp -R ./migrations ./target/$WINDOWS_DESTINATION/
cp -R ./migrations ./target/$LINUX_DESTINATION/

cp ./.example.env ./target/$WINDOWS_DESTINATION/.env
cp ./.example.env ./target/$LINUX_DESTINATION/.env
