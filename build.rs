use std::env;

fn main() {
    let root = String::from(env::var_os("CARGO_MANIFEST_DIR").unwrap().to_str().unwrap());

    // 将 /lib 添加到链接搜索目录
    println!("cargo:rustc-link-search=native={}", format!("{}/lib", &root));

    // 引入c++动态库（macOS）
    println!("cargo:rustc-link-lib=dylib=c++");
}