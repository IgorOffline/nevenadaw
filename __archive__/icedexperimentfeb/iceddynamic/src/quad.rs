use bytemuck::{Pod, Zeroable};
use iced::wgpu;

pub const INITIAL_INSTANCES: usize = 2_000;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Quad {
    pub pos: [f32; 2],
    pub scale: [f32; 2],
    pub border_color: [f32; 4],
    pub border_radius: [f32; 4],
    pub border_width: f32,
    pub shadow_color: [f32; 4],
    pub shadow_offset: [f32; 2],
    pub shadow_blur_radius: f32,
    pub snap: u32,
}

pub fn color_target_state(format: wgpu::TextureFormat) -> [Option<wgpu::ColorTargetState>; 1] {
    [Some(wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })]
}
