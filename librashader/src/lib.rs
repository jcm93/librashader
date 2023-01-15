#![forbid(missing_docs)]
//! RetroArch shader preset compiler and runtime.
//!
//! librashader provides convenient and safe access to RetroArch ['slang' shaders](https://github.com/libretro/slang-shaders).
//! The preset parser, shader preprocessor, and shader runtimes have all been reimplemented in Rust to provide easy access to
//! the rich library of shaders.
//!
//! ## Usage
//! The core objects in librashader are the [`ShaderPreset`](crate::presets::ShaderPreset) and the
//! filter chain implementations.
//!
//! The basic workflow involves parsing a `ShaderPreset`, which can then be used to construct
//! a `FilterChain`. All shaders will then be compiled, after which `FilterChain::frame` can be
//! called with appropriate input and output parameters to draw a frame with the shader effect applied.
//!
//! ## Runtimes
//! Currently available runtimes are Vulkan 1.3+, OpenGL 3.3+ and 4.6 (with DSA), and Direct3D 11.
//! Work on the Direct3D 12 runtimes are in progress. The Vulkan runtime requires [`VK_KHR_dynamic_rendering`](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html).
//!
//! | **API**     | **Status** | **`librashader` feature** |
//! |-------------|------------|---------------------------|
//! | OpenGL 3.3+ | ✔         | `gl`                     |
//! | OpenGL 4.6  | ✔         | `gl`                     |
//! | Vulkan      | ✔         | `vk`                     |
//! | Direct3D 11  | ✔         | `d3d11`                  |
//! | Direct3D 12  | 🚧         | `d3d12`                  |
//! | OpenGL 2    | ❌         |                          |
//! | DirectX 9   | ❌         |                          |
//! | Metal       | ❌         |                          |
//!
//! ## C API
//! For documentation on the librashader C API, see [librashader_capi](https://docs.rs/librashader-capi/latest/librashader_capi/),
//! or [`librashader.h`](https://github.com/SnowflakePowered/librashader/blob/master/include/librashader.h).

#[cfg(feature = "presets")]
/// Parsing and usage of shader presets.
///
/// Shader presets contain shader and texture parameters, and the order in which to apply a set of shaders
/// in a filter chain.
pub mod presets {
    use librashader_preprocess::{PreprocessError, ShaderParameter, ShaderSource};
    pub use librashader_presets::*;
    /// Get full parameter metadata from a shader preset.
    pub fn get_parameter_meta(
        preset: &ShaderPreset,
    ) -> Result<impl Iterator<Item = ShaderParameter>, PreprocessError> {
        let iters: Result<Vec<Vec<ShaderParameter>>, PreprocessError> = preset
            .shaders
            .iter()
            .map(|s| ShaderSource::load(&s.name).map(|s| s.parameters.into_values().collect()))
            .into_iter()
            .collect();
        let iters = iters?;
        Ok(iters.into_iter().flatten())
    }
}

#[cfg(feature = "preprocess")]
/// Loading and preprocessing of 'slang' shader source files.
///
/// Shader sources files must be loaded with imports resolved before being able to be compiled.
/// Shader parameters are also defined in `#pragma`s within shader source files which must be parsed.
pub mod preprocess {
    pub use librashader_preprocess::*;
}

#[cfg(feature = "reflect")]
/// Shader compilation and reflection.
pub mod reflect {
    /// Supported shader compiler targets.
    pub mod targets {
        pub use librashader_reflect::back::targets::GLSL;
        pub use librashader_reflect::back::targets::HLSL;
        pub use librashader_reflect::back::targets::SPIRV;
    }

    pub use librashader_reflect::error::*;

    pub use librashader_reflect::reflect::{semantics, ReflectShader, ShaderReflection};

    pub use librashader_reflect::back::{
        targets::OutputTarget, CompileShader, CompilerBackend, FromCompilation,
        ShaderCompilerOutput,
    };
    pub use librashader_reflect::front::shaderc::GlslangCompilation;
    pub use librashader_reflect::reflect::semantics::BindingMeta;

    /// Helpers to deal with image loading.
    pub mod image {
        pub use librashader_runtime::image::*;
    }
}

/// Shader runtimes to execute a filter chain on a GPU surface.
#[cfg(feature = "runtime")]
pub mod runtime {
    pub use librashader_common::{Size, Viewport};
    pub use librashader_runtime::parameters::FilterChainParameters;

    #[cfg(feature = "gl")]
    /// Shader runtime for OpenGL 3.3+.
    ///
    /// Note that the OpenGL runtime requires `gl` to be
    /// initialized with [`gl::load_with`](https://docs.rs/gl/0.14.0/gl/fn.load_with.html).
    pub mod gl {
        pub use librashader_runtime_gl::{
            error,
            options::{FilterChainOptionsGL as FilterChainOptions, FrameOptionsGL as FrameOptions},
            FilterChainGL as FilterChain, Framebuffer, GLImage,
        };

        #[doc(hidden)]
        /// Re-exports names to deal with C API conflicts.
        ///
        /// This is internal to librashader-capi and is exempt from semantic versioning.
        pub mod capi {
            pub use librashader_runtime_gl::*;
        }
    }

    #[cfg(feature = "d3d11")]
    /// Shader runtime for Direct3D 11.
    pub mod d3d11 {
        pub use librashader_runtime_d3d11::{
            error,
            options::{
                FilterChainOptionsD3D11 as FilterChainOptions, FrameOptionsD3D11 as FrameOptions,
            },
            D3D11InputView, D3D11OutputView, FilterChainD3D11 as FilterChain,
        };

        #[doc(hidden)]
        /// Re-exports names to deal with C API conflicts.
        ///
        /// This is internal to librashader-capi and is exempt from semantic versioning.
        pub mod capi {
            pub use librashader_runtime_d3d11::*;
        }
    }

    #[cfg(feature = "vk")]
    /// Shader runtime for Vulkan 1.3+.
    pub mod vk {
        pub use librashader_runtime_vk::{
            error,
            options::{
                FilterChainOptionsVulkan as FilterChainOptions, FrameOptionsVulkan as FrameOptions,
            },
            FilterChainVulkan as FilterChain, VulkanImage, VulkanInstance, VulkanObjects,
        };

        #[doc(hidden)]
        /// Re-exports names to deal with C API conflicts.
        ///
        /// This is internal to librashader-capi and is exempt from semantic versioning.
        pub mod capi {
            pub use librashader_runtime_vk::*;
        }
    }

    #[doc(hidden)]
    /// Helper methods for runtimes.
    ///
    /// This is internal to librashader runtimes and is exempt from semantic versioning.
    pub mod helper {
        pub use librashader_runtime::semantics::insert_lut_semantics;
        pub use librashader_runtime::semantics::insert_pass_semantics;
    }
}

pub use librashader_common::{FilterMode, ImageFormat, WrapMode};
