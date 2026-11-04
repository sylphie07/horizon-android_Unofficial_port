fn main() {
    println!("cargo:rerun-if-env-changed=SYLPHRA_BUILD_CHANNEL");
    println!("cargo:rustc-check-cfg=cfg(sylphra_development)");
    let channel = std::env::var("SYLPHRA_BUILD_CHANNEL").unwrap_or_else(|_| "stable".into());
    if !matches!(channel.as_str(), "stable" | "beta" | "development") {
        panic!("SYLPHRA_BUILD_CHANNEL must be stable, beta, or development");
    }
    println!("cargo:rustc-env=SYLPHRA_BUILD_CHANNEL={channel}");
    if matches!(channel.as_str(), "beta" | "development") {
        println!("cargo:rustc-cfg=sylphra_development");
    }
    // Android tombstones can match a native frame to the exact private symbols.
    // This affects the final ELF only, not code generation or release optimization.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        println!("cargo:rustc-link-arg=-Wl,--build-id=sha1");
    }
}
