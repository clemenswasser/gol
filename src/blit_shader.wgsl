struct Camera {
    zoom_level: f32,
    offset_x: f32,
    offset_y: f32,
}

@group(0) @binding(0) 
var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) vertex_position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
}

fn apply_perspective(pos: vec2<f32>) -> vec4<f32> {
    return vec4((pos + vec2(-camera.offset_x, camera.offset_y)) * camera.zoom_level, 0.0, 1.0);
}

@vertex
fn vs_main(@builtin(vertex_index) ix: u32) -> VertexOutput {
    var vertex_output: VertexOutput;
    switch ix {
        case 0u: {
            vertex_output.vertex_position = apply_perspective(vec2(-1.0, 1.0));
            vertex_output.texture_coordinates = vec2(0.0, 0.0);
        }
        case 1u: {
            vertex_output.vertex_position = apply_perspective(vec2(-1.0, -1.0));
            vertex_output.texture_coordinates = vec2(0.0, 1.0);
        }
        case 2u: {
            vertex_output.vertex_position = apply_perspective(vec2(1.0, -1.0));
            vertex_output.texture_coordinates = vec2(1.0, 1.0);
        }
        case 3u: {
            vertex_output.vertex_position = apply_perspective(vec2(-1.0, 1.0));
            vertex_output.texture_coordinates = vec2(0.0, 0.0);
        }
        case 4u: {
            vertex_output.vertex_position = apply_perspective(vec2(1.0, -1.0));
            vertex_output.texture_coordinates = vec2(1.0, 1.0);
        }
        case 5u: {
            vertex_output.vertex_position = apply_perspective(vec2(1.0, 1.0));
            vertex_output.texture_coordinates = vec2(1.0, 0.0);
        }
        default: {}
    }
    return vertex_output;
}
            
@group(0) @binding(1)
var texture_input: texture_2d<f32>;
@group(0) @binding(2)
var texture_sampler: sampler;
            
@fragment
fn fs_main(vertex_input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_input, texture_sampler, vertex_input.texture_coordinates);
}