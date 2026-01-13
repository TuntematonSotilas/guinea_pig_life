# guinea_pig_life
Guinea Pig life

## Install

* Add Rust targets : `rustup target add x86_64-linux-android aarch64-linux-android`
* On windows install the `C++ MSVC Build Tool` from Visual Studio
* Install cargo-ndk : `cargo install cargo-ndk` 
* Install gradle : https://gradle.org and add in Path
* Install Android Studio and add env variables : `ANDROID_HOME` and `ANDROID_SDK_ROOT`
* Install the `Android SDK Command-line Tools` from Android Studio (`Tool > Android SDK > SDK Tool`)
* Accept Android NDK licence `sdkmanager --licenses` 
* Generate gradle/wrapper, gradlew, and gradle.bat with: `gradle wrapper`

## Build

`cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --package guinea_pig_life`

## Debug 

Open the folder `app` with Android Studio and Run

