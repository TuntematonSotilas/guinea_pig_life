# guinea_pig_life

Guinea pig life 

A game where you play a Guinea Pig

![icon](app/src/main/res/mipmap-hdpi/ic_launcher.webp)

## Install

#### Rust

* Install Rust : https://rust-lang.org/fr/tools/install/
* Add Rust targets : `rustup target add x86_64-linux-android aarch64-linux-android`
* On Windows install the `C++ MSVC Build Tool` from Visual Studio
* On Fedora install the bevy requirements : `sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel`
* Install cargo-ndk : `cargo install cargo-ndk` 


#### Android

* Install Android Studio and add env variables : `ANDROID_HOME` ; `ANDROID_SDK_ROOT` ; `JAVA_HOME`
* Install gradle : https://gradle.org and add in Path
* Install the `Android SDK Command-line Tools` from Android Studio (`Tool > Android SDK > SDK Tool`)
* Accept Android NDK licence `sdkmanager --licenses` 
* Generate gradle/wrapper and gradlew with: `gradle wrapper`

Example of .barshrc : 
```
export JAVA_HOME="$HOME/android-studio/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_SDK_ROOT="$HOME/Android/Sdk"
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig/"
alias gradle="$HOME/gradle/bin/gradle"
```

## Build

Build : `cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --package guinea_pig_life`

## Run 

To run on Android : Open the folder `app` with Android Studio and Run

To run on Desktop : `cargo run`

## Palette 

https://colorhunt.co/palette/a8df8ef0ffdfffd8dfffaab8
