#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import bevy_render::view::View;

// Parallax material bindings
struct ParallaxMaterial {
    color: vec4<f32>,
    depth: vec2<f32>,
    offset: vec2<f32>,
    repeat_scale: vec2<f32>,
}

// Camera view
@group(0) @binding(0) var<uniform> view: View;

// Bindings from material
@group(2) @binding(0) var<uniform> parallax_material: ParallaxMaterial;
@group(2) @binding(1) var base_texture: texture_2d<f32>;
@group(2) @binding(2) var base_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate the camera offset with depth
    let camera_offset = (view.world_position.xy + parallax_material.offset) * parallax_material.depth;

    // Get texture color with correct repeating of the texture
    let base_color = textureSample(base_texture, base_sampler, (camera_offset + in.uv) * parallax_material.repeat_scale);

    // Output the color tinted by the material color
    return base_color * parallax_material.color;
}
