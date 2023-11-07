#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput
#import bevy_sprite::mesh2d_view_bindings view
#import bevy_sprite::mesh2d_bindings
#import bevy_sprite::mesh2d_functions mesh2d_position_world_to_clip

struct Uniforms {
    // scale for background movement inside viewport
    movement_scale: f32,
    // webgl2 requires 16 byte alignment
    _wasm_padding: vec3<f32>
};

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

@fragment
fn fragment(
    in: FullscreenVertexOutput,
) -> @location(0) vec4<f32> {
    let movement_scale = uniforms.movement_scale;
    let offset = mesh2d_position_world_to_clip(vec4<f32>(view.world_position.xy, 0.0, 0.0)).xy;
    let color = scroll(texture, texture_sampler, movement_scale, in.uv, offset, view.viewport.zw, view.view_proj);
    return color;
}

fn scroll(
    texture: texture_2d<f32>,
    texture_sampler: sampler,
    movement_scale: f32,
    uv: vec2<f32>,
    offset: vec2<f32>,
    viewport_size: vec2<f32>,
    view_proj: mat4x4<f32>,
) -> vec4<f32> {
    let offset = vec2<f32>(-offset.x, offset.y);

    var uv = uv - (offset * movement_scale);
    let tex_dim = textureDimensions(texture);

    uv = uv * (viewport_size / vec2<f32>(tex_dim));
    let color = textureSample(texture, texture_sampler, uv);
    return color;
}