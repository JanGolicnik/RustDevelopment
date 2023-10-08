#import bevy_pbr::mesh_vertex_output MeshVertexOutput

@group(1) @binding(0) var<uniform> time: f32;
@group(1) @binding(1) var<uniform> resolution: vec2<f32>;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {

    var uv = mesh.uv * 2.0 - 1.0;
    uv.x *= resolution.x / resolution.y;

    let uv0 = uv;

    var final_color = vec3(0.0); 

    for (var i = 0.0; i < 2.0; i += 1.0) {

        uv = fract(uv * (1.5 + sin(time))) - 0.5;

        var d  = length(uv) * exp(-length(uv0));

        var col = pallete(length(uv0) + time * 3.0 + i);    

        d = sin(d * 8.0 + time) / 8.0;
        d = abs(d);
        d = smoothstep(0.0, 0.1, d);
        d = (abs(sin(time)) * 0.03) / d;
        d = pow(d, 1.2);

        col *= d;

        final_color += col;
    }


    return vec4(final_color, 1.0);//output_color;
}

fn pallete(t : f32) -> vec3<f32> {
    let a = vec3(0.5, 0.5, 0.5);
    let b = vec3(0.5, 0.5, 0.5);
    let c = vec3(1.0, 1.0, 1.0);
    let d = vec3(0.263, 0.416, 0.557);

    return a + b * cos( 6.28318 * (c * t * d));
}