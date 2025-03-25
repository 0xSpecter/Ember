
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct InstanceInput {
    @location(5) mm0: vec4<f32>, // Model matrix
    @location(6) mm1: vec4<f32>,
    @location(7) mm2: vec4<f32>,
    @location(8) mm3: vec4<f32>,

    @location(9) nm0: vec3<f32>, // Normal matrix
    @location(10) nm1: vec3<f32>,
    @location(11) nm2: vec3<f32>,
}


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>
}

@group(1) @binding(0) 
var<uniform> camera: Camera;

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

    let normal_matrix = mat3x3<f32>(
        instance.nm0,
        instance.nm1,
        instance.nm2,
    );

    out.world_normal = normal_matrix * model.normal;
    out.tex_coords = model.tex_coords;

    var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
    
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;

    return out;
}

@group(0) @binding(0)
var tx: texture_2d<f32>;

@group(0) @binding(1)
var sp: sampler;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(3) @binding(0)
var<uniform> light: Light;

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    let obj_color: vec4<f32> = textureSample(tx, sp, in.tex_coords);

    let ambient_strength = 0.05;
    let ambient_color = light.color * ambient_strength;

    let falloff = max(distance(in.world_position, light.position) / 2.0, 1.0);

    let light_direction = normalize(light.position - in.world_position);
    let diffuse_strength = max(dot(in.world_normal, light_direction), 0.0);
    let diffuse_color = light.color * diffuse_strength / falloff;

    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_direction);
    let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color / falloff;


    let result = (ambient_color + diffuse_color + specular_color)  * obj_color.xyz;

    return vec4<f32>(result, obj_color.a);
}
