# miracle-focus-blur-plugin

![Screenshot of miracle-focus-blur-plugin in action](screenshot.png)

A [miracle-wm](https://github.com/mattkae/miracle-wm) plugin that blurs unfocused windows using a two-pass separable Gaussian blur shader.

When a window loses focus it receives a horizontal-then-vertical Gaussian blur (radius 15px, sigma 5). When it regains focus the blur is removed. Attached (tiled) windows are unaffected.

## Installation

Download and install the latest nightly build:

```sh
curl -fsSL https://raw.githubusercontent.com/miracle-wm-org/miracle-focus-blur-plugin/main/install.sh | bash
```

This places `miracle_focus_blur_plugin.wasm` in `$XDG_CONFIG_HOME/miracle-wm/plugins` (defaults to `~/.config/miracle-wm/plugins`), which will be automatically loaded by miracle-wm.

```yaml
plugins:
  - ~/.config/miracle-wm/plugins/miracle_focus_blur_plugin.wasm
```

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
