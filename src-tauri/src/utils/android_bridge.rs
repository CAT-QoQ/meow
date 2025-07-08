use jni::objects::{JClass, JValue};
use jni::{JNIEnv, JavaVM};
use once_cell::sync::OnceCell;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn set_java_vm(vm: JavaVM) {
    if GLOBAL_JVM.get().is_some() {
        // 如果已经初始化，直接返回
        println!("JavaVM is already set, skipping initialization");
        return;
    }
    GLOBAL_JVM.set(vm).expect("JavaVM can only be set once");
}

pub fn get_java_vm() -> &'static JavaVM {
    GLOBAL_JVM.get().expect("JavaVM is not initialized")
}

#[no_mangle]
pub extern "system" fn Java_com_meow_app_MainActivity_setRustJNIEnv(
    env: JNIEnv<'static>,
    _class: JClass,
) {
    // 将 JavaVM 转换为 Rust 的全局 JavaVM 对象
    // 获取 JavaVM
    let java_vm = env.get_java_vm().expect("Failed to get JavaVM");
    // 全局存储 JavaVM
    set_java_vm(java_vm);
}

pub fn get_attach_guard() -> Result<jni::AttachGuard<'static>, String> {
    let java_vm = get_java_vm();
    java_vm
        .attach_current_thread()
        .map_err(|e| format!("Failed to attach thread: {:?}", e))
}

// 从全局环境中调用 Java 方法
pub fn toggle_status_bar_theme(is_dark_mode: bool) -> Result<bool, String> {
    println!("Current Thread ID (set): {:?}", std::thread::current().id());

    // 获取当前线程的 JNIEnv
    // let mut guard = get_attach_guard()?;
    // let mut env = &mut *guard; // 解引用为 `JNIEnv`
    let mut env = get_attach_guard()?;


    // 找到 MainActivity 类
    let main_activity_class = env
        .find_class("com/meow/app/MainActivity")
        .map_err(|e| format!("Failed to find class: {:?}", e))?;

    // 调用静态方法 getInstance 获取 Activity 实例
    let activity_instance = env
        .call_static_method(
            main_activity_class,
            "getInstance",
            "()Lcom/meow/app/MainActivity;",
            &[],
        )
        .map_err(|e| format!("Failed to get instance: {:?}", e))?
        .l()
        .map_err(|_| "Expected JObject")?;

    // 调用 `yourMethodName`
    let result = env
        .call_method(
            activity_instance,
            "toggleStatusBarTextColor", // 方法名
            "(Z)Z",                     // 方法签名：接收一个布尔值，返回一个布尔值
            &[JValue::from(is_dark_mode)],
        )
        .map_err(|e| format!("Failed to call method: {:?}", e))?
        .z()
        .map_err(|_| "Expected boolean return value")?;

    Ok(result)
}
