use miracle_plugin::plugin::{register_window_shader, Plugin};
use miracle_plugin::window::Window;
use std::sync::OnceLock;

// Two-pass separable Gaussian blur. A 2D Gaussian factors into a horizontal
// then a vertical 1D pass, turning an O(radius^2) single-pass blur into
// O(radius): pass 0 blurs along x, pass 1 reads that result as `tex` and blurs
// along y. Radius/sigma are in pixels, so the blur is consistent at any window
// size. Each tap is clamped to the texture edge so border texels don't wrap.
const RADIUS: f32 = 15.0;
const SIGMA: f32 = 5.0;

fn blur_pass(axis: &str) -> String {
    format!(
        "
vec4 sample_to_rgba(in vec2 texcoord) {{
    const float RADIUS = {RADIUS:.1};
    const float SIGMA  = {SIGMA:.1};

    vec2 texel = 1.0 / surfaceSize;
    vec2 lo = 0.5 * texel;
    vec2 hi = 1.0 - 0.5 * texel;
    vec4 color = vec4(0.0);
    float total = 0.0;

    for (float i = -RADIUS; i <= RADIUS; i += 1.0) {{
        float w = exp(-(i * i) / (2.0 * SIGMA * SIGMA));
        vec2 uv = clamp(texcoord + {axis} * texel, lo, hi);
        color += texture2D(tex, uv) * w;
        total += w;
    }}
    return color / total;
}}
"
    )
}

static BLUR_SHADER_ID: OnceLock<Option<u8>> = OnceLock::new();

fn blur_shader_id() -> Option<u8> {
    *BLUR_SHADER_ID.get_or_init(|| {
        let horizontal = blur_pass("vec2(i, 0.0)");
        let vertical = blur_pass("vec2(0.0, i)");
        register_window_shader(&[&horizontal, &vertical])
    })
}

#[derive(Default)]
struct FocusBlurPlugin;

impl Plugin for FocusBlurPlugin {
    fn window_focused(&mut self, window: &Window) {
        if window.state == miracle_plugin::window::WindowState::Attached {
            return;
        }

        let _ = window.set_shader(None);
    }

    fn window_unfocused(&mut self, window: &Window) {
        if let Some(id) = blur_shader_id() {
            if window.state == miracle_plugin::window::WindowState::Attached {
                return;
            }
            let _ = window.set_shader(Some(id));
        }
    }
}

miracle_plugin::miracle_plugin!(FocusBlurPlugin);
