#[cfg(target_os = "macos")]
pub fn set_process_icon() {
    use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy, NSImage};
    use cocoa::base::{id, nil, YES};
    use cocoa::foundation::NSString;

    let icon_data = match crate::server::Assets::get("app_icon.icns") {
        Some(d) => d,
        None => return,
    };

    // 写入临时文件后通过路径加载，比 NSData 内存加载更可靠
    let tmp_path = std::env::temp_dir().join("ohscrcpy_icon.icns");
    if std::fs::write(&tmp_path, &icon_data.data).is_err() {
        return;
    }

    unsafe {
        let path_str = NSString::alloc(nil).init_str(&tmp_path.to_string_lossy());
        let ns_image: id = NSImage::alloc(nil);
        let ns_image = ns_image.initByReferencingFile_(path_str);
        if ns_image == nil {
            let _ = std::fs::remove_file(&tmp_path);
            return;
        }

        let app = NSApplication::sharedApplication(nil);
        if app == nil {
            let _ = std::fs::remove_file(&tmp_path);
            return;
        }

        // 脱离父终端的 Dock 分组，独立显示
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
        app.activateIgnoringOtherApps_(YES);
        app.setApplicationIconImage_(ns_image);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn set_process_icon() {}
