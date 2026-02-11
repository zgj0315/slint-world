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

## Cursor Dev
```
分析当前工程的架构，在此工程基础上，设计如下功能：
实现读取 android 系统信息；
将信息显示在屏幕上；

```