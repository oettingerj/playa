fn main() {
    println!("cargo:rustc-link-lib=framework=NotificationCenter");
    tauri_build::build()
}
