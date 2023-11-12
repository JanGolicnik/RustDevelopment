#import bevy_pbr::{
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world, mesh_normal_local_to_world},
    pbr_types::{PbrInput, pbr_input_new},
    pbr_functions as fns,
    mesh_view_bindings::view,
}
#import bevy_core_pipeline::tonemapping::tone_mapping

@group(1) @binding(0) var world_texture: texture_2d_array<f32>;
@group(1) @binding(1) var world_texure_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) texture_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) texture_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    var out: VertexOutput;

    out.uv = vertex.uv;
    out.texture_index = vertex.texture_index;

    var model = get_model_matrix(vertex.instance_index);

    out.clip_position = mesh_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    return out;
}

struct FragmentInput {
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) texture_index: u32,
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    @builtin(front_facing) is_front: bool,
    input: FragmentInput,
) -> @location(0) vec4<f32> {

    var pbr_input: PbrInput = pbr_input_new();

    pbr_input.material.base_color = textureSample(world_texture, world_texure_sampler, input.uv, input.texture_index);

    pbr_input.frag_coord = position;
    pbr_input.world_position = input.world_position;
    pbr_input.world_normal = fns::prepare_world_normal(
        input.world_normal,
        false,
        is_front,
    );

    pbr_input.is_orthographic = false;

    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        input.world_normal,
        false,
        is_front,
        input.uv,
        view.mip_bias,
    );
    pbr_input.V = fns::calculate_view(input.world_position, pbr_input.is_orthographic);

    return tone_mapping(fns::apply_pbr_lighting(pbr_input), view.color_grading);
}