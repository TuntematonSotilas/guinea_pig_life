# guinea_pig_life
Guinea Pig life

## Install

* Add Rust targets : `rustup target add x86_64-linux-android`
* On windows install the `C++ MSVC Build Tool`
* Install cargo-ndk : `cargo install cargo-ndk` 
* Install gradle : https://gradle.org

## Build

* cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --package guinea_pig_life