//! Binding types for the librashader C API.
use crate::error::LibrashaderError;
use librashader::presets::context::{Orientation, VideoDriver, WildcardContext};
use librashader::presets::ShaderPreset;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

/// A handle to a shader preset object.
pub type libra_shader_preset_t = Option<NonNull<ShaderPreset>>;

/// A handle to a preset wildcard context object.
pub type libra_preset_ctx_t = Option<NonNull<WildcardContext>>;

/// A handle to a librashader error object.
pub type libra_error_t = Option<NonNull<LibrashaderError>>;

/// An enum representing orientation for use in preset contexts.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LIBRA_PRESET_CTX_ORIENTATION {
    Vertical = 0,
    Horizontal,
}
impl From<LIBRA_PRESET_CTX_ORIENTATION> for Orientation {
    fn from(value: LIBRA_PRESET_CTX_ORIENTATION) -> Self {
        match value {
            LIBRA_PRESET_CTX_ORIENTATION::Vertical => Orientation::Vertical,
            LIBRA_PRESET_CTX_ORIENTATION::Horizontal => Orientation::Horizontal,
        }
    }
}

// An enum representing graphics runtimes (video drivers) for use in preset contexts.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LIBRA_PRESET_CTX_RUNTIME {
    None = 0,
    GlCore,
    Vulkan,
    D3D11,
    D3D12,
    Metal,
}

impl From<LIBRA_PRESET_CTX_RUNTIME> for VideoDriver {
    fn from(value: LIBRA_PRESET_CTX_RUNTIME) -> Self {
        match value {
            LIBRA_PRESET_CTX_RUNTIME::None => VideoDriver::None,
            LIBRA_PRESET_CTX_RUNTIME::GlCore => VideoDriver::GlCore,
            LIBRA_PRESET_CTX_RUNTIME::Vulkan => VideoDriver::Vulkan,
            LIBRA_PRESET_CTX_RUNTIME::D3D11 => VideoDriver::Direct3D11,
            LIBRA_PRESET_CTX_RUNTIME::D3D12 => VideoDriver::Direct3D12,
            LIBRA_PRESET_CTX_RUNTIME::Metal => VideoDriver::Metal,
        }
    }
}

#[cfg(feature = "runtime-opengl")]
use librashader::runtime::gl::FilterChain as FilterChainGL;

/// A handle to a OpenGL filter chain.
#[cfg(feature = "runtime-opengl")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "runtime-opengl")))]
pub type libra_gl_filter_chain_t = Option<NonNull<FilterChainGL>>;

/// A handle to a Direct3D 11 filter chain.
#[cfg(any(
    feature = "__cbindgen_internal",
    all(target_os = "windows", feature = "runtime-d3d11")
))]
use librashader::runtime::d3d11::FilterChain as FilterChainD3D11;

/// A handle to a Direct3D 11 filter chain.
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(target_os = "windows", feature = "runtime-d3d11")))
)]
#[cfg(any(
    feature = "__cbindgen_internal",
    all(target_os = "windows", feature = "runtime-d3d11")
))]
pub type libra_d3d11_filter_chain_t = Option<NonNull<FilterChainD3D11>>;

#[cfg(any(
    feature = "__cbindgen_internal",
    all(target_os = "windows", feature = "runtime-d3d12")
))]
use librashader::runtime::d3d12::FilterChain as FilterChainD3D12;
/// A handle to a Direct3D 12 filter chain.
#[cfg(any(
    feature = "__cbindgen_internal",
    all(target_os = "windows", feature = "runtime-d3d12")
))]
pub type libra_d3d12_filter_chain_t = Option<NonNull<FilterChainD3D12>>;

/// A handle to a Direct3D 9 filter chain.
#[cfg(any(
    feature = "__cbindgen_internal",
    all(target_os = "windows", feature = "runtime-d3d9")
))]
use librashader::runtime::d3d9::FilterChain as FilterChainD3D9;

/// A handle to a Direct3D 11 filter chain.
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(target_os = "windows", feature = "runtime-d3d9")))
)]
#[cfg(any(
    feature = "__cbindgen_internal",
    all(target_os = "windows", feature = "runtime-d3d9")
))]
pub type libra_d3d9_filter_chain_t = Option<NonNull<FilterChainD3D9>>;

#[cfg(feature = "runtime-vulkan")]
use librashader::runtime::vk::FilterChain as FilterChainVulkan;
/// A handle to a Vulkan filter chain.
#[cfg(feature = "runtime-vulkan")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "runtime-vulkan")))]
pub type libra_vk_filter_chain_t = Option<NonNull<FilterChainVulkan>>;

#[cfg(all(target_os = "macos", feature = "runtime-metal"))]
use librashader::runtime::mtl::FilterChain as FilterChainMetal;
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(target_vendor = "apple", feature = "runtime-metal")))
)]
#[cfg(any(
    feature = "__cbindgen_internal",
    all(
        target_vendor = "apple",
        feature = "runtime-metal",
        feature = "__cbindgen_internal_objc"
    )
))]
pub type libra_mtl_filter_chain_t = Option<NonNull<FilterChainMetal>>;

/// Defines the output viewport for a rendered frame.
#[repr(C)]
pub struct libra_viewport_t {
    /// The x offset in the viewport framebuffer to begin rendering from.
    pub x: f32,
    /// The y offset in the viewport framebuffer to begin rendering from.
    pub y: f32,
    /// The width of the viewport framebuffer.
    pub width: u32,
    /// The height of the viewport framebuffer.
    pub height: u32,
}

pub(crate) trait FromUninit<T>
where
    Self: Sized,
{
    fn from_uninit(value: MaybeUninit<Self>) -> T;
}

macro_rules! config_set_field {
    (@POINTER $options:ident.$field:ident <- $ptr:ident) => {
        $options.$field = unsafe { ::std::ptr::addr_of!((*$ptr).$field).read() };
    };
    (@POINTER @NEGATIVE $options:ident.$field:ident <- $ptr:ident) => {
        $options.$field = unsafe { !::std::ptr::addr_of!((*$ptr).$field).read() };
    };
    (@LITERAL $options:ident.$field:ident <- $value:literal) => {
        $options.$field = $value;
    };
}

macro_rules! config_version_set {
    // "optimized" version for normal behaviour
    (@ROOT $realver:ident $version:literal => [$($field:ident),+ $(,)?] ($options:ident <- $ptr:ident)) => {
        #[allow(unused_comparisons)]
        if $realver >= $version {
            $($crate::ctypes::config_set_field!(@POINTER $options.$field <- $ptr);)+
        }
    };

    // Repeater
    (@ROOT $realver:ident $version:literal => [$($field:tt),+ $(,)?] ($options:ident <- $ptr:ident)) => {
        $(crate::ctypes::config_version_set!(@SINGLE $realver $version => [$field] ($options <- $ptr));)+
    };

    // Allow overriding default value with a literal for older versions
    (@SINGLE $realver:ident $version:literal => [($field:ident: $value:literal)] ($options:ident <- $ptr:ident)) => {
        #[allow(unused_comparisons)]
        if $realver >= $version {
            $crate::ctypes::config_set_field!(@LITERAL $options.$field <- $value);
        }
    };

    // Allow negation of prior variables that is version dependent.
    (@SINGLE $realver:ident $version:literal => [(!$field:ident)] ($options:ident <- $ptr:ident)) => {
        #[allow(unused_comparisons)]
        if $realver >= $version {
            $crate::ctypes::config_set_field!(@POINTER @NEGATIVE $options.$field <- $ptr);
        }
    };

    (@SINGLE $realver:ident $version:literal => [$field:ident] ($options:ident <- $ptr:ident)) => {
        #[allow(unused_comparisons)]
        if $realver >= $version {
            $crate::ctypes::config_set_field!(@POINTER $options.$field <- $ptr);
        }
    };
}

/// Macro to declare a configuration struct, with options to change behaviour based on
/// API version.
///
/// For example following declaration does the following
///
/// * Declare `frames_in_flight`, `use_dynamic_rendering` for API version 0, with the following forward compatibility statements
///     * Inverts the behaviour of `use_dynamic_rendering` compared to API version 1.
///     * `disable_cache` is defaulted to `true` for API version 0, regardless of `Default::default`
///        but is not declared for API 0.
/// * Declare `use_dynamic_rendering` with normal behaviour, and `disable_cache` for API version 1.
/// * All fields that are undeclared inherit `Default::default`
///
/// ```rust
/// config_struct! {
///     impl FilterChainOptions => filter_chain_vk_opt_t {
///         0 => [frames_in_flight, (!use_dynamic_rendering), (disable_cache: true)];
///         1 => [use_dynamic_rendering, disable_cache];
///     }
/// }
/// ```
macro_rules! config_struct {
    (impl $rust:ty => $capi:ty {$($version:literal => [$($field:tt),+]);+ $(;)?}) => {
        impl $crate::ctypes::FromUninit<$rust> for $capi {
            fn from_uninit(value: ::std::mem::MaybeUninit<Self>) -> $rust {
                let ptr = value.as_ptr();
                let version = unsafe { ::std::ptr::addr_of!((*ptr).version).read() };

                let mut options = <$rust>::default();
                $(
                    $crate::ctypes::config_version_set!(@ROOT version $version => [$($field),+] (options <- ptr));
                )+
                options
            }
        }
    }
}

pub(crate) use config_set_field;
pub(crate) use config_struct;
pub(crate) use config_version_set;

#[doc(hidden)]
#[deny(deprecated)]
#[deprecated = "Forward declarations for cbindgen, do not use."]
mod __cbindgen_opaque_forward_declarations {
    macro_rules! typedef_struct {
        ($($(#[$($attrss:tt)*])* $name:ident;)*) => {
            $($(#[$($attrss)*])*
                #[allow(unused)]
                #[doc(hidden)]
                #[deny(deprecated)]
                #[deprecated]
                pub struct $name;
            )*
        };
    }

    typedef_struct! {
        /// Opaque struct for a preset context.
        WildcardContext;
        /// Opaque struct for a shader preset.
        ShaderPreset;
        /// Opaque struct for an OpenGL filter chain.
        FilterChainGL;
        /// Opaque struct for a Direct3D 11 filter chain.
        FilterChainD3D11;
        /// Opaque struct for a Direct3D 12 filter chain.
        FilterChainD3D12;
        /// Opaque struct for a Direct3D 9 filter chain.
        FilterChainD3D9;
        /// Opaque struct for a Vulkan filter chain.
        FilterChainVulkan;
        /// Opaque struct for a Metal filter chain.
        FilterChainMetal;
    }
}
