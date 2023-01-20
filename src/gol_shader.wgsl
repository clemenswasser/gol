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
@workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    var active_cells = u32(0);

    active_cells += u32(global_id.x > u32(0) && global_id.y > u32(0) && cell_is_active(global_id.xy + vec2(u32(-1), u32(-1))));
    active_cells += u32(global_id.y > u32(0) && cell_is_active(global_id.xy + vec2(u32(0), u32(-1))));
    active_cells += u32(global_id.x < num_workgroups.x - u32(1) && global_id.y > u32(0) && cell_is_active(global_id.xy + vec2(u32(1), u32(-1))));
    active_cells += u32(global_id.x > u32(0) && cell_is_active(global_id.xy + vec2(u32(-1), u32(0))));
    active_cells += u32(global_id.x < num_workgroups.x - u32(1) && cell_is_active(global_id.xy + vec2(u32(1), u32(0))));
    active_cells += u32(global_id.x > u32(0) && global_id.y < num_workgroups.y - u32(1) && cell_is_active(global_id.xy + vec2(u32(-1), u32(1))));
    active_cells += u32(global_id.y < num_workgroups.y - u32(1) && cell_is_active(global_id.xy + vec2(u32(0), u32(1))));
    active_cells += u32(global_id.x < num_workgroups.x - u32(1) && global_id.y < num_workgroups.y - u32(1) && cell_is_active(global_id.xy + vec2(u32(1), u32(1))));

    var cell_color = vec4(0.0, 0.0, 0.0, 1.0);

    if active_cells == u32(3) || (active_cells == u32(2) && cell_is_active(global_id.xy)) {
        cell_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
    textureStore(front_texture, vec2<i32>(global_id.xy), cell_color);
}