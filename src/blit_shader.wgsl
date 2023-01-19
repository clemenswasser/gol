@vertex
fn vs_main(@builtin(vertex_index) ix: u32) -> @builtin(position) vec4<f32> {
                // Generate a full screen quad in NDCs
    var vertex = vec2(-1.0, 1.0);
    switch ix {
                    case 1u: {
            vertex = vec2(-1.0, -1.0);
        }
                    case 2u, 4u: {
            vertex = vec2(1.0, -1.0);
        }
                    case 5u: {
            vertex = vec2(1.0, 1.0);
        }
                    default: {}
                }
    return vec4(vertex, 0.0, 1.0);
}
            
@group(0) @binding(0)
var texture_input: texture_2d<f32>;
            
@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let rgba_sep = textureLoad(texture_input, vec2<i32>(pos.xy), 0);
    return vec4(rgba_sep.rgb * rgba_sep.a, rgba_sep.a);
}