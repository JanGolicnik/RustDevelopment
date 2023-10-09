#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct StandardMaterial {
    time: f32,
};

@group(1) @binding(0)
var<uniform> material: StandardMaterial;

@group(1) @binding(1)
var texture: texture_2d<f32>;

@group(1) @binding(2)
var our_sampler: sampler;

@fragment
fn fragment(
    in: MeshVertexOutput
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    var output_color = textureSample(texture, our_sampler, uv).xyz;

    return vec4(output_color, 1.0);
}