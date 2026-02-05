use bytemuck::{Pod, Zeroable};

pub mod color {
    use super::*;

    #[derive(Debug, Clone, Copy, Pod, Zeroable)]
    #[repr(C)]
    pub struct Packed([f32; 4]);

    impl From<iced::Color> for Packed {
        fn from(color: iced::Color) -> Self {
            Self(color.into_linear())
        }
    }
}

pub mod gradient {
    use super::*;

    #[derive(Debug, Clone, Copy, Pod, Zeroable)]
    #[repr(C)]
    pub struct Packed {
        pub colors_1: [u32; 4],
        pub colors_2: [u32; 4],
        pub colors_3: [u32; 4],
        pub colors_4: [u32; 4],
        pub offsets: [u32; 4],
        pub direction: [f32; 4],
    }
}
