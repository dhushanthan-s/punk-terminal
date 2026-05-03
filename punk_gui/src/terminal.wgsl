struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) fg: vec3<f32>,
    @location(2) bg: vec3<f32>,
}

@vertex
fn vs_main(
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) fg: vec3<f32>,
    @location(3) bg: vec3<f32>,
) -> VsOut {
    var o: VsOut;
    o.clip_pos = vec4<f32>(pos, 0.0, 1.0);
    o.uv = uv;
    o.fg = fg;
    o.bg = bg;
    return o;
}

@group(0) @binding(0)
var font_tex: texture_2d<f32>;
@group(0) @binding(1)
var font_samp: sampler;

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let cov = textureSample(font_tex, font_samp, in.uv).r;
    let rgb = mix(in.bg, in.fg, cov);
    return vec4<f32>(rgb, 1.0);
}
