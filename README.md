# focus-blur-plugin

![Screenshot of focus-blur-plugin in action](screenshot.png)

A [miracle-wm](https://github.com/mattkae/miracle-wm) plugin that blurs unfocused windows using a two-pass separable Gaussian blur shader.

When a window loses focus it receives a horizontal-then-vertical Gaussian blur (radius 15px, sigma 5). When it regains focus the blur is removed. Attached (tiled) windows are unaffected.

## Installation

Download and install the latest stable release:

```sh
curl -fsSL https://raw.githubusercontent.com/miracle-wm-org/focus-blur-plugin/main/install.sh | bash
```

### Nightly build (secondary)

For the latest unreleased changes from `main`, install the nightly build instead:

```sh
curl -fsSL https://raw.githubusercontent.com/miracle-wm-org/focus-blur-plugin/main/install.sh | bash -s -- --nightly
```

Alternatively, manually add the plugin to your miracle-wm configuration file (`~/.config/miracle-wm/config.yaml`):

```yaml
plugins:
  - /path/to/focus-blur-plugin/target/wasm32-wasip1/release/focus_blur_plugin.wasm
```

## Building

### Prerequisites
```sh
sudo apt-get install -y libmircore-dev clang libclang-dev
rustup target add wasm32-wasip1
```

### Compilation
```sh
cargo build --target wasm32-wasip1 --release
```
The compiled WASM file can be found at `target/wasm32-wasip1/release/focus_blur_plugin.wasm`.
