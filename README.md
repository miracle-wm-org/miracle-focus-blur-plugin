# miracle-focus-blur-plugin

![Screenshot of miracle-focus-blur-plugin in action](screenshot.png)

A [miracle-wm](https://github.com/mattkae/miracle-wm) plugin that blurs unfocused windows using a two-pass separable Gaussian blur shader.

When a window loses focus it receives a horizontal-then-vertical Gaussian blur (radius 15px, sigma 5). When it regains focus the blur is removed. Attached (tiled) windows are unaffected.

## Prerequisites

- Rust toolchain (stable)

## Build

```sh
cargo build --target wasm32-wasip1 --release
```

The compiled plugin will be at `target/wasm32-wasip1/release/miracle_focus_blur_plugin.wasm`.

## Usage in miracle-wm

Add the plugin to your miracle-wm configuration file (`~/.config/miracle-wm/config.yaml`):

```yaml
plugins:
  - /path/to/miracle-focus-blur-plugin/target/wasm32-wasip1/release/miracle_focus_blur_plugin.wasm
```

Then reload the miracle-wm configuration for the plugin to take effect.
