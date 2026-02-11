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
    let mut get_build_info = |field: &str| -> String {
        let val = env
            .get_static_field("android/os/Build", field, "Ljava/lang/String;")
            .and_then(|f| f.l())
            .unwrap_or(JObject::null());
        if val.is_null() {
            return "Unknown".into();
        }
        env.get_string(&val.into())
            .map(|s| s.into())
            .unwrap_or_else(|_| "Error".into())
    };

    // 4. 读取具体信息
    let manufacturer = get_build_info("MANUFACTURER"); // 厂商
    let model = get_build_info("MODEL"); // 型号
    let device = get_build_info("DEVICE"); // 设备代号
    let board = get_build_info("BOARD"); // 主板/芯片组代号

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

    let context_obj = unsafe { JObject::from_raw(ctx.context().cast()) };
    // 读取 Wi-Fi 信息 ---
    let mut get_wifi_info = || -> Result<String, jni::errors::Error> {
        // 1. 创建字符串 "wifi" 用于查询服务
        let service_name = env.new_string("wifi")?;

        // 2. 调用 Context.getSystemService("wifi") -> 返回 Object (WifiManager)
        let wifi_manager = env
            .call_method(
                &context_obj,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[(&service_name).into()],
            )?
            .l()?;

        if wifi_manager.is_null() {
            return Ok("WifiManager not found".to_string());
        }

        // 3. 调用 WifiManager.getConnectionInfo() -> 返回 WifiInfo
        let wifi_info = env
            .call_method(
                &wifi_manager,
                "getConnectionInfo",
                "()Landroid/net/wifi/WifiInfo;",
                &[],
            )?
            .l()?;

        if wifi_info.is_null() {
            return Ok("No Wifi Connection".to_string());
        }

        // 4. 读取 WifiInfo 的字段

        // --- 获取 SSID (Wi-Fi 名称) ---
        let ssid_obj = env
            .call_method(&wifi_info, "getSSID", "()Ljava/lang/String;", &[])?
            .l()?;
        let ssid = env
            .get_string(&ssid_obj.into())
            .map(|s| s.into())
            .unwrap_or_else(|_| "<unknown>".to_string());

        // --- 获取 RSSI (信号强度) ---
        let rssi = env.call_method(&wifi_info, "getRssi", "()I", &[])?.i()?;

        // --- 获取 LinkSpeed (连接速度 Mbps) ---
        let link_speed = env
            .call_method(&wifi_info, "getLinkSpeed", "()I", &[])?
            .i()?;

        // --- 获取 IP 地址 (int) 并转换 ---
        let ip_int = env
            .call_method(&wifi_info, "getIpAddress", "()I", &[])?
            .i()?;
        let ip_str = format!(
            "{}.{}.{}.{}",
            ip_int & 0xFF,
            (ip_int >> 8) & 0xFF,
            (ip_int >> 16) & 0xFF,
            (ip_int >> 24) & 0xFF
        );

        Ok(format!(
            "SSID: {}\nSignal: {} dBm\nSpeed: {} Mbps\nIP: {}",
            ssid, rssi, link_speed, ip_str
        ))
    };

    // 执行 Wi-Fi 读取并处理错误
    let wifi_details = get_wifi_info().unwrap_or_else(|e| format!("Wifi Error: {:?}", e));
    format!(
        "Manufacturer: {}\nModel: {}\nAndroid Version: {}\nDevice: {}\nBoard: {}\n[Wi-Fi Info]\n{}",
        manufacturer, model, version_release, device, board, wifi_details
    )
}

#[cfg(not(target_os = "android"))]
fn get_android_system_info() -> String {
    "Not running on Android\n(Simulated Data)".to_string()
}
