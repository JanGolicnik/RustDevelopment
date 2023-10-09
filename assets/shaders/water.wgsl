#import bevy_pbr::mesh_view_bindings   globals
#import bevy_pbr::mesh_bindings        mesh
#import bevy_pbr::mesh_functions       get_model_matrix, mesh_position_local_to_world, mesh_normal_local_to_world, mesh_position_local_to_clip

@group(1) @binding(0) var<uniform> time: f32;
@group(1) @binding(1) var<uniform> cam_pos: vec3<f32>;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) view: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    var wavelength = 2.6;
    var amplitude = 0.033;
    var speed = 5.0;
    var direction = vec3(0.5, 0.0, 0.5);

    var out: VertexOutput;

    var model = mesh.model;

    out.uv = vertex.uv;

    var position = vertex.position;
    var tangent = vec3(1.0, 0.0, 0.0);
    var bitangent = vec3(0.0, 0.0, -1.0);

    var d  = 0.0;

    for(var i = 0.0; i < 24.0; i += 1.0)
    {
        let h = (dot(direction, vec3(position.x + d, 0.0, position.z))) * wavelength + time * speed;

        position.y += amplitude * sin(h);
        
        d = amplitude * cos(h);

        tangent.y += d;
        bitangent.y += d;

        wavelength *= 1.18;
        amplitude *= 0.83;
        speed *= .99;
        let neki = vec2(i + 0.1, i + 0.2);
        direction = vec3(gold_noise(neki), 0.0, gold_noise(vec2(neki.y, neki.x)));
        direction = normalize(direction);
    }

    out.clip_position = mesh_position_local_to_clip(model, vec4<f32>(position, 1.0)); 
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));

    out.world_normal = normalize(normalize(cross(tangent, bitangent)));
    
    out.view = normalize(cam_pos - vec3(out.world_position.x, out.world_position.y, out.world_position.z)); 

    return out;
}

struct FragmentInput {
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) view: vec3<f32>,
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    input: FragmentInput,
) -> @location(0) vec4<f32> {

    let light_dir = normalize(vec3(0.0, 1.0, 3.5));

    let angle = max(dot(light_dir, normalize(input.world_normal)), 0.0);

    let reflection = normalize(2.0 * dot(input.world_normal, light_dir) * input.world_normal - light_dir); 

    let specular = pow(max(dot(reflection, input.view), 0.0), 150.0) * 1.0;

    let diffuseColor = vec3(0.3, 0.4, 1.0);
    let specularColor = vec3(1.0, 1.0, 0.5); 
    let ambientColor = vec3(0.94, 1.0, 1.0); 

    var finalColor = (diffuseColor * (ambientColor + angle)) + (specularColor * specular);

    return vec4(finalColor, 1.0);
}

const PHI = 1.61803398874989484820459; 

fn gold_noise(xy: vec2<f32>) -> f32
{
    return fract( tan( distance(xy * PHI, xy )) * xy.x);
}