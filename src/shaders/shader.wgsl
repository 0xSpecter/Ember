
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(5) mm0: vec4<f32>,
    @location(6) mm1: vec4<f32>,
    @location(7) mm2: vec4<f32>,
    @location(8) mm3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>
}

@group(1) @binding(0) 
var<uniform> camera: CameraUniform;

@group(2) @binding(0)
var<uniform> time: f32;


@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = mat4x4<f32>(
        instance.mm0,
        instance.mm1,
        instance.mm2,
        instance.mm3,
    );

    let scale: f32 = (sin(time / 1000) + 2) / 10;
    let sc = mat4x4<f32>(
        vec4<f32>(scale, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, scale, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );

    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);

    return out;
}

@group(0) @binding(0)
var tx: texture_2d<f32>;

@group(0) @binding(1)
var sp: sampler;

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return textureSample(tx, sp, in.tex_coords);
}
