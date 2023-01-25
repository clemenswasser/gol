@group(0)
@binding(0)
var front_texture: texture_storage_2d<rgba8unorm, write>;

@group(0)
@binding(1)
var back_texture: texture_storage_2d<rgba8unorm, read>;

fn cell_is_active(cell_pos: vec2<u32>) -> bool {
    let inactive_cell = vec3(0.0, 0.0, 0.0);
    return all(textureLoad(back_texture, vec2<i32>(cell_pos)).xyz != inactive_cell);
}

@compute
@workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let active_cells = u32(cell_is_active(global_id.xy + vec2(u32(-1), u32(-1)))) + u32(cell_is_active(global_id.xy + vec2(u32(0), u32(-1)))) + u32(cell_is_active(global_id.xy + vec2(u32(1), u32(-1)))) + u32(cell_is_active(global_id.xy + vec2(u32(-1), u32(0)))) + u32(cell_is_active(global_id.xy + vec2(u32(1), u32(0)))) + u32(cell_is_active(global_id.xy + vec2(u32(-1), u32(1)))) + u32(cell_is_active(global_id.xy + vec2(u32(0), u32(1)))) + u32(cell_is_active(global_id.xy + vec2(u32(1), u32(1))));
    let cell_active = active_cells == u32(3) || (active_cells == u32(2) && cell_is_active(global_id.xy));
    textureStore(front_texture, vec2<i32>(global_id.xy), vec4(vec3(1.0) * f32(cell_active), 1.0));
}