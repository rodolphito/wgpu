//! Bindings to the WebGPU API.
//!
//! Internally vendored from the `web-sys` crate until the WebGPU binding are stabilized.
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

#![allow(unused_imports, non_snake_case)]
use web_sys::{Event, EventTarget};
mod gen_Gpu;
pub use gen_Gpu::*;
mod gen_GpuAdapter;
pub use gen_GpuAdapter::*;
mod gen_GpuAdapterInfo;
pub use gen_GpuAdapterInfo::*;
mod gen_GpuAddressMode;
pub use gen_GpuAddressMode::*;
mod gen_GpuAutoLayoutMode;
pub use gen_GpuAutoLayoutMode::*;
mod gen_GpuBindGroup;
pub use gen_GpuBindGroup::*;
mod gen_GpuBindGroupDescriptor;
pub use gen_GpuBindGroupDescriptor::*;
mod gen_GpuBindGroupEntry;
pub use gen_GpuBindGroupEntry::*;
mod gen_GpuBindGroupLayout;
pub use gen_GpuBindGroupLayout::*;
mod gen_GpuBindGroupLayoutDescriptor;
pub use gen_GpuBindGroupLayoutDescriptor::*;
mod gen_GpuBindGroupLayoutEntry;
pub use gen_GpuBindGroupLayoutEntry::*;
mod gen_GpuBlendComponent;
pub use gen_GpuBlendComponent::*;
mod gen_GpuBlendFactor;
pub use gen_GpuBlendFactor::*;
mod gen_GpuBlendOperation;
pub use gen_GpuBlendOperation::*;
mod gen_GpuBlendState;
pub use gen_GpuBlendState::*;
mod gen_GpuBuffer;
pub use gen_GpuBuffer::*;
mod gen_GpuBufferBinding;
pub use gen_GpuBufferBinding::*;
mod gen_GpuBufferBindingLayout;
pub use gen_GpuBufferBindingLayout::*;
mod gen_GpuBufferBindingType;
pub use gen_GpuBufferBindingType::*;
mod gen_GpuBufferDescriptor;
pub use gen_GpuBufferDescriptor::*;
mod gen_GpuBufferMapState;
pub use gen_GpuBufferMapState::*;
mod gen_GpuCanvasAlphaMode;
pub use gen_GpuCanvasAlphaMode::*;
mod gen_GpuCanvasContext;
pub use gen_GpuCanvasContext::*;
mod gen_GpuCanvasConfiguration;
pub use gen_GpuCanvasConfiguration::*;
mod gen_GpuCanvasToneMapping;
pub use gen_GpuCanvasToneMapping::*;
mod gen_GpuCanvasToneMappingMode;
pub use gen_GpuCanvasToneMappingMode::*;
mod gen_GpuColorDict;
pub use gen_GpuColorDict::*;
mod gen_GpuColorTargetState;
pub use gen_GpuColorTargetState::*;
mod gen_GpuCommandBuffer;
pub use gen_GpuCommandBuffer::*;
mod gen_GpuCommandBufferDescriptor;
pub use gen_GpuCommandBufferDescriptor::*;
mod gen_GpuCommandEncoder;
pub use gen_GpuCommandEncoder::*;
mod gen_GpuCommandEncoderDescriptor;
pub use gen_GpuCommandEncoderDescriptor::*;
mod gen_GpuCompareFunction;
pub use gen_GpuCompareFunction::*;
mod gen_GpuCompilationInfo;
pub use gen_GpuCompilationInfo::*;
mod gen_GpuCompilationMessage;
pub use gen_GpuCompilationMessage::*;
mod gen_GpuCompilationMessageType;
pub use gen_GpuCompilationMessageType::*;
mod gen_GpuComputePassDescriptor;
pub use gen_GpuComputePassDescriptor::*;
mod gen_GpuComputePassEncoder;
pub use gen_GpuComputePassEncoder::*;
mod gen_GpuComputePassTimestampWrites;
pub use gen_GpuComputePassTimestampWrites::*;
mod gen_GpuComputePipeline;
pub use gen_GpuComputePipeline::*;
mod gen_GpuComputePipelineDescriptor;
pub use gen_GpuComputePipelineDescriptor::*;
mod gen_GpuCullMode;
pub use gen_GpuCullMode::*;
mod gen_GpuDepthStencilState;
pub use gen_GpuDepthStencilState::*;
mod gen_GpuDevice;
pub use gen_GpuDevice::*;
mod gen_GpuDeviceDescriptor;
pub use gen_GpuDeviceDescriptor::*;
mod gen_GpuDeviceLostInfo;
pub use gen_GpuDeviceLostInfo::*;
mod gen_GpuDeviceLostReason;
pub use gen_GpuDeviceLostReason::*;
mod gen_GpuError;
pub use gen_GpuError::*;
mod gen_GpuErrorFilter;
pub use gen_GpuErrorFilter::*;
mod gen_GpuExternalTexture;
pub use gen_GpuExternalTexture::*;
mod gen_GpuExternalTextureBindingLayout;
pub use gen_GpuExternalTextureBindingLayout::*;
mod gen_GpuExternalTextureDescriptor;
pub use gen_GpuExternalTextureDescriptor::*;
mod gen_GpuExtent3dDict;
pub use gen_GpuExtent3dDict::*;
mod gen_GpuFeatureName;
pub use gen_GpuFeatureName::*;
mod gen_GpuFilterMode;
pub use gen_GpuFilterMode::*;
mod gen_GpuFragmentState;
pub use gen_GpuFragmentState::*;
mod gen_GpuFrontFace;
pub use gen_GpuFrontFace::*;
mod gen_GpuTexelCopyBufferInfo;
pub use gen_GpuTexelCopyBufferInfo::*;
mod gen_GpuCopyExternalImageSourceInfo;
pub use gen_GpuCopyExternalImageSourceInfo::*;
mod gen_GpuTexelCopyTextureInfo;
pub use gen_GpuTexelCopyTextureInfo::*;
mod gen_GpuCopyExternalImageDestInfo;
pub use gen_GpuCopyExternalImageDestInfo::*;
mod gen_GpuTexelCopyBufferLayout;
pub use gen_GpuTexelCopyBufferLayout::*;
mod gen_GpuIndexFormat;
pub use gen_GpuIndexFormat::*;
mod gen_GpuLoadOp;
pub use gen_GpuLoadOp::*;
mod gen_gpu_map_mode;
pub use gen_gpu_map_mode::*;
mod gen_GpuMipmapFilterMode;
pub use gen_GpuMipmapFilterMode::*;
mod gen_GpuMultisampleState;
pub use gen_GpuMultisampleState::*;
mod gen_GpuObjectDescriptorBase;
pub use gen_GpuObjectDescriptorBase::*;
mod gen_GpuOrigin2dDict;
pub use gen_GpuOrigin2dDict::*;
mod gen_GpuOrigin3dDict;
pub use gen_GpuOrigin3dDict::*;
mod gen_GpuOutOfMemoryError;
pub use gen_GpuOutOfMemoryError::*;
mod gen_GpuPipelineDescriptorBase;
pub use gen_GpuPipelineDescriptorBase::*;
mod gen_GpuPipelineLayout;
pub use gen_GpuPipelineLayout::*;
mod gen_GpuPipelineLayoutDescriptor;
pub use gen_GpuPipelineLayoutDescriptor::*;
mod gen_GpuPowerPreference;
pub use gen_GpuPowerPreference::*;
mod gen_GpuPrimitiveState;
pub use gen_GpuPrimitiveState::*;
mod gen_GpuPrimitiveTopology;
pub use gen_GpuPrimitiveTopology::*;
mod gen_GpuProgrammableStage;
pub use gen_GpuProgrammableStage::*;
mod gen_GpuQuerySet;
pub use gen_GpuQuerySet::*;
mod gen_GpuQuerySetDescriptor;
pub use gen_GpuQuerySetDescriptor::*;
mod gen_GpuQueryType;
pub use gen_GpuQueryType::*;
mod gen_GpuQueue;
pub use gen_GpuQueue::*;
mod gen_GpuQueueDescriptor;
pub use gen_GpuQueueDescriptor::*;
mod gen_GpuRenderBundle;
pub use gen_GpuRenderBundle::*;
mod gen_GpuRenderBundleDescriptor;
pub use gen_GpuRenderBundleDescriptor::*;
mod gen_GpuRenderBundleEncoder;
pub use gen_GpuRenderBundleEncoder::*;
mod gen_GpuRenderBundleEncoderDescriptor;
pub use gen_GpuRenderBundleEncoderDescriptor::*;
mod gen_GpuRenderPassColorAttachment;
pub use gen_GpuRenderPassColorAttachment::*;
mod gen_GpuRenderPassDepthStencilAttachment;
pub use gen_GpuRenderPassDepthStencilAttachment::*;
mod gen_GpuRenderPassDescriptor;
pub use gen_GpuRenderPassDescriptor::*;
mod gen_GpuRenderPassEncoder;
pub use gen_GpuRenderPassEncoder::*;
mod gen_GpuRenderPassTimestampWrites;
pub use gen_GpuRenderPassTimestampWrites::*;
mod gen_GpuRenderPipeline;
pub use gen_GpuRenderPipeline::*;
mod gen_GpuRenderPipelineDescriptor;
pub use gen_GpuRenderPipelineDescriptor::*;
mod gen_GpuRequestAdapterOptions;
pub use gen_GpuRequestAdapterOptions::*;
mod gen_GpuSampler;
pub use gen_GpuSampler::*;
mod gen_GpuSamplerBindingLayout;
pub use gen_GpuSamplerBindingLayout::*;
mod gen_GpuSamplerBindingType;
pub use gen_GpuSamplerBindingType::*;
mod gen_GpuSamplerDescriptor;
pub use gen_GpuSamplerDescriptor::*;
mod gen_GpuShaderModule;
pub use gen_GpuShaderModule::*;
mod gen_GpuShaderModuleDescriptor;
pub use gen_GpuShaderModuleDescriptor::*;
mod gen_GpuStencilFaceState;
pub use gen_GpuStencilFaceState::*;
mod gen_GpuStencilOperation;
pub use gen_GpuStencilOperation::*;
mod gen_GpuStorageTextureAccess;
pub use gen_GpuStorageTextureAccess::*;
mod gen_GpuStorageTextureBindingLayout;
pub use gen_GpuStorageTextureBindingLayout::*;
mod gen_GpuStoreOp;
pub use gen_GpuStoreOp::*;
mod gen_GpuSupportedFeatures;
pub use gen_GpuSupportedFeatures::*;
mod gen_GpuSupportedLimits;
pub use gen_GpuSupportedLimits::*;
mod gen_GpuTexture;
pub use gen_GpuTexture::*;
mod gen_GpuTextureAspect;
pub use gen_GpuTextureAspect::*;
mod gen_GpuTextureBindingLayout;
pub use gen_GpuTextureBindingLayout::*;
mod gen_GpuTextureDescriptor;
pub use gen_GpuTextureDescriptor::*;
mod gen_GpuTextureDimension;
pub use gen_GpuTextureDimension::*;
mod gen_GpuTextureFormat;
pub use gen_GpuTextureFormat::*;
mod gen_GpuTextureSampleType;
pub use gen_GpuTextureSampleType::*;
mod gen_GpuTextureView;
pub use gen_GpuTextureView::*;
mod gen_GpuTextureViewDescriptor;
pub use gen_GpuTextureViewDescriptor::*;
mod gen_GpuTextureViewDimension;
pub use gen_GpuTextureViewDimension::*;
mod gen_GpuUncapturedErrorEvent;
pub use gen_GpuUncapturedErrorEvent::*;
mod gen_GpuUncapturedErrorEventInit;
pub use gen_GpuUncapturedErrorEventInit::*;
mod gen_GpuValidationError;
pub use gen_GpuValidationError::*;
mod gen_GpuVertexAttribute;
pub use gen_GpuVertexAttribute::*;
mod gen_GpuVertexBufferLayout;
pub use gen_GpuVertexBufferLayout::*;
mod gen_GpuVertexFormat;
pub use gen_GpuVertexFormat::*;
mod gen_GpuVertexState;
pub use gen_GpuVertexState::*;
mod gen_GpuVertexStepMode;
pub use gen_GpuVertexStepMode::*;
mod gen_WgslLanguageFeatures;
pub use gen_WgslLanguageFeatures::*;
