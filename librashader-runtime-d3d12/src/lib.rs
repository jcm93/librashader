#![feature(const_trait_impl)]
#![feature(let_chains)]
#![feature(type_alias_impl_trait)]
mod error;
mod filter_chain;
mod descriptor_heap;
mod hello_triangle;
mod samplers;
mod luts;
mod util;
mod mipmap;
mod filter_pass;
mod quad_render;
mod graphics_pipeline;
mod buffer;
mod framebuffer;
mod texture;
mod render_target;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hello_triangle::{DXSample, SampleCommandLine};

    #[test]
    fn triangle_d3d12() {
        let sample = hello_triangle::d3d12_hello_triangle::Sample::new(
            // "../test/slang-shaders/crt/crt-royale.slangp",
            "../test/slang-shaders/bezel/Mega_Bezel/Presets/MBZ__0__SMOOTH-ADV.slangp",
            &SampleCommandLine {
                use_warp_device: false,
            },
        )
        .unwrap();
        hello_triangle::main(sample).unwrap()
    }
}
