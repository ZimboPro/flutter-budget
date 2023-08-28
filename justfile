default: gen lint

gen:
    flutter pub get
    flutter_rust_bridge_codegen

lint:
    cd native && cargo fmt
    dart format .

clean:
    flutter clean
    cd native && cargo clean
    
serve *args='':
    flutter pub run flutter_rust_bridge:serve {{args}}

android-setup:
    rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android \
        i686-linux-android
    cargo install cargo-ndk
    echo 'Run the following with the appropriate changes'
    echo 'echo "ANDROID_NDK=(path to NDK)" >> ~/.gradle/gradle.properties'

android:
    cd native && cargo ndk -o ../android/app/src/main/jniLibs build
    flutter run


# vim:expandtab:sw=4:ts=4
