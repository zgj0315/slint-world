use std::error::Error;

use slint::SharedString;

slint::include_modules!();

pub fn run() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let info = get_android_system_info();
    ui.set_sys_info(SharedString::from(info));
    ui.run()?;
    Ok(())
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    run().unwrap();
}

#[cfg(target_os = "android")]
fn get_android_system_info() -> String {
    use jni::JavaVM;
    use jni::objects::JObject;

    // 1. 获取 Android 上下文中的 JavaVM
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.expect("Failed to get JavaVM");

    // 2. 附加到当前线程以获取 JNI 环境
    let mut env = vm.attach_current_thread().expect("Failed to attach thread");

    // 3. 定义一个辅助闭包来读取 android.os.Build 的静态字符串字段
    let mut get_field = |field_name: &str| -> String {
        let field_val = env
            .get_static_field("android/os/Build", field_name, "Ljava/lang/String;")
            .and_then(|f| f.l()) // 转为对象
            .unwrap_or(JObject::null());

        if field_val.is_null() {
            return "Unknown".to_string();
        }

        // 将 Java String 转换为 Rust String
        env.get_string(&field_val.into())
            .map(|s| s.into())
            .unwrap_or_else(|_| "Error".to_string())
    };

    // 4. 读取具体信息
    let manufacturer = get_field("MANUFACTURER"); // 厂商
    let model = get_field("MODEL"); // 型号
    let device = get_field("DEVICE"); // 设备代号
    let board = get_field("BOARD"); // 主板/芯片组代号

    // 读取 android.os.Build.VERSION.RELEASE (嵌套静态类)
    let version_release = env
        .get_static_field("android/os/Build$VERSION", "RELEASE", "Ljava/lang/String;")
        .and_then(|f| f.l())
        .map(|obj| {
            env.get_string(&obj.into())
                .map(|s| s.into())
                .unwrap_or_default()
        })
        .unwrap_or_else(|_| "Unknown".to_string());

    format!(
        "Manufacturer: {}\nModel: {}\nAndroid Version: {}\nDevice: {}\nBoard: {}",
        manufacturer, model, version_release, device, board
    )
}

#[cfg(not(target_os = "android"))]
fn get_android_system_info() -> String {
    "Not running on Android\n(Simulated Data)".to_string()
}
