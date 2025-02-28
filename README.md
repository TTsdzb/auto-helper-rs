# auto-helper-rs

用于简化各种自动化操作的 Rust 库。

## 在 Windows 中使用 Enigo

如果你要在 Windows 平台使用 Enigo 提供模拟输入，需要为你的可执行文件添加 Manifest。

添加构建依赖：

```shell
cargo add --build embed-manifest
```

在 `Cargo.toml` 同目录下创建 `build.rs`，写入以下内容：

```rust
use embed_manifest::{embed_manifest, manifest::DpiAwareness, new_manifest};

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(new_manifest("Contoso.Sample").dpi_awareness(DpiAwareness::PerMonitor))
            .expect("Unable to embed manifest file for Windows.");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
```
