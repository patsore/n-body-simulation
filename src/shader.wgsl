var<private> VERTICES: array<vec3<f32>, 3> = array<vec3<f32>, 3>(
    vec3<f32>(-1.7321,-1.0, 0.0),
    vec3<f32>( 1.7321,-1.0, 0.0), // sqrt(3) ≈ 1.7321
    vec3<f32>( 0.0, 2.0, 0.0),
);

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) color: vec4<f32>,
  @location(1) local_position: vec2<f32>,
};

struct VertexInput {
   @builtin(vertex_index) vertex_index: u32,
};

struct CircleInstance{
  @location(0) world_pos: vec3<f32>,
  @location(1) radius: f32,
  @location(2) color: u32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

fn unpack_u32_to_vec4(value: u32) -> vec4<f32> {
    // Bitwise AND with 0xFF to extract the individual bytes
    let r = (value >> 24u) & 0xFFu;
    let g = (value >> 16u) & 0xFFu;
    let b = (value >> 8u) & 0xFFu;
    let a = value & 0xFFu;

    // Normalize the values to the range [0.0, 1.0]
    let max_value = 255.0;
    let result = vec4<f32>(
        f32(r) / max_value,
        f32(g) / max_value,
        f32(b) / max_value,
        f32(a) / max_value,
    );

    return result;
}


@vertex
fn vs_main(
  @builtin(vertex_index) vertex_index: u32,
  instance: CircleInstance,
) -> VertexOutput {
  var out: VertexOutput;

  let local_position = VERTICES[vertex_index];

  out.clip_position = vec4<f32>((local_position  + instance.world_pos) * instance.radius, 1.0) * camera.view_proj;
  out.local_position = vec2<f32>(local_position.x, local_position.y);
  out.color = unpack_u32_to_vec4(instance.color);

  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if dot(in.local_position, in.local_position) > 1.0 {
//        return vec4<f32>(1.0, 0.0, 1.0, 1.0);
        discard;
    }
    return in.color;
}