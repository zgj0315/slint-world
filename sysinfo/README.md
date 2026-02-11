## Desktop
```
# dev
cargo run

# package
cargo packager -r

```

## Android Apk
```
# list device
x devices

# run on device
x run --device adb:30906b3e

# package apk
x build --platform android --arch arm64 --format apk --release

```
