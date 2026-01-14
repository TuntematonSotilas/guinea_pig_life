# guinea_pig_life

Guinea pig life 

A game where you play a Guinea Pig

![icon](app/src/main/res/mipmap-hdpi/ic_launcher.webp)

## Install

* Add Rust targets : `rustup target add x86_64-linux-android aarch64-linux-android`
* On windows install the `C++ MSVC Build Tool` from Visual Studio
* Install cargo-ndk : `cargo install cargo-ndk` 
* Install Android Studio and add env variables : `ANDROID_HOME` ; `ANDROID_SDK_ROOT` ; `JAVA_HOME`
* Install gradle : https://gradle.org and add in Path
* Install the `Android SDK Command-line Tools` from Android Studio (`Tool > Android SDK > SDK Tool`)
* Accept Android NDK licence `sdkmanager --licenses` 
* Generate gradle/wrapper, gradlew, and gradle.bat with: `gradle wrapper`

Example of .barshrc : 
```
export JAVA_HOME="$HOME/android-studio/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_SDK_ROOT="$HOME/Android/Sdk"
alias gradle="$HOME/gradle/bin/gradle"
```

## Build

`cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --package guinea_pig_life`

## Debug 

Open the folder `app` with Android Studio and Run


## Palette 

https://colorhunt.co/palette/a8df8ef0ffdfffd8dfffaab8
