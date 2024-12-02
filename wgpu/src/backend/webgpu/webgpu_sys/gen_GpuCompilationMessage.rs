// DO NOT EDIT THIS FILE!
//
// This module part of a subset of web-sys that is used by wgpu's webgpu backend.
//
// These bindings are vendored into wgpu for the sole purpose of letting
// us pin the WebGPU backend to a specific version of the bindings, not
// to enable local changes. There are no provisions to preserve changes
// you make here the next time we re-vendor the bindings.
//
// The `web-sys` crate does not treat breaking changes to the WebGPU API
// as semver breaking changes, as WebGPU is "unstable". This means Cargo
// will not let us mix versions of `web-sys`, pinning WebGPU bindings to
// a specific version, while letting other bindings like WebGL get
// updated. Vendoring WebGPU was the workaround we chose.
//
// Vendoring also allows us to avoid building `web-sys` with
// `--cfg=web_sys_unstable_apis`, needed to get the WebGPU bindings.
//
// If you want to improve the generated code, please submit a PR to the https://github.com/rustwasm/wasm-bindgen repository.
//
// This file was generated by the `cargo xtask vendor-web-sys --version 0.2.97` command.
#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUCompilationMessage , typescript_type = "GPUCompilationMessage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCompilationMessage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCompilationMessage;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUCompilationMessage" , js_name = message)]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn message(this: &GpuCompilationMessage) -> ::alloc::string::String;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUCompilationMessage" , js_name = type)]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`, `GpuCompilationMessageType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &GpuCompilationMessage) -> GpuCompilationMessageType;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUCompilationMessage" , js_name = lineNum)]
    #[doc = "Getter for the `lineNum` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/lineNum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn line_num(this: &GpuCompilationMessage) -> f64;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUCompilationMessage" , js_name = linePos)]
    #[doc = "Getter for the `linePos` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/linePos)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn line_pos(this: &GpuCompilationMessage) -> f64;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUCompilationMessage" , js_name = offset)]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn offset(this: &GpuCompilationMessage) -> f64;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUCompilationMessage" , js_name = length)]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn length(this: &GpuCompilationMessage) -> f64;
}
