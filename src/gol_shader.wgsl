@group(0)
@binding(0)
var front_texture: texture_storage_2d<rgba8unorm, write>;

@group(0)
@binding(1)
var back_texture: texture_storage_2d<rgba8unorm, read>;

@compute
@workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let start_x = u32(max(0, i32(global_id.x) - 1));
    let end_x = min(global_id.x + u32(1), num_workgroups.x);
    let start_y = u32(max(0, i32(global_id.y) - 1));
    let end_y = min(global_id.y + u32(1), num_workgroups.y);
    let inactive_cell = vec3(0.0, 0.0, 0.0);

    var active_cells = u32(0);

    for (var y: u32 = start_y; y <= end_y; y += u32(1)) {
        for (var x: u32 = start_x; x <= end_x; x += u32(1)) {
            active_cells += u32((x != global_id.x || y != global_id.y) && all(textureLoad(back_texture, vec2(i32(x), i32(y))).xyz != inactive_cell));
        }
    }

    let pixel_pos = vec2<i32>(global_id.xy);
    var cell_color = vec4(0.0, 0.0, 0.0, 1.0);

    if active_cells == u32(3) || (active_cells == u32(2) && all(textureLoad(back_texture, pixel_pos).xyz != inactive_cell)) {
        cell_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
    textureStore(front_texture, pixel_pos, cell_color);
}