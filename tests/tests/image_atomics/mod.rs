//! Tests for image atomics.

use wgpu::ShaderModuleDescriptor;
use wgpu_test::{gpu_test, GpuTestConfiguration, TestParameters, TestingContext};

#[gpu_test]
static IMAGE_32_ATOMICS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(
        TestParameters::default()
            .limits(wgt::Limits {
                max_storage_textures_per_shader_stage: 1,
                max_compute_invocations_per_workgroup: 64,
                max_compute_workgroup_size_x: 4,
                max_compute_workgroup_size_y: 4,
                max_compute_workgroup_size_z: 4,
                max_compute_workgroups_per_dimension: wgt::COPY_BYTES_PER_ROW_ALIGNMENT,
                ..wgt::Limits::downlevel_webgl2_defaults()
            })
            .features(
                wgpu::Features::TEXTURE_ATOMIC
                    | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            ),
    )
    .run_async(|ctx| async move {
        test_format(
            ctx,
            wgpu::TextureFormat::R32Uint,
            wgpu::include_wgsl!("image_32_atomics.wgsl"),
        )
        .await;
    });

async fn test_format(
    ctx: TestingContext,
    format: wgpu::TextureFormat,
    desc: ShaderModuleDescriptor<'_>,
) {
    let pixel_bytes = format.target_pixel_byte_cost().unwrap();
    let size = wgpu::Extent3d {
        width: wgt::COPY_BYTES_PER_ROW_ALIGNMENT,
        height: wgt::COPY_BYTES_PER_ROW_ALIGNMENT,
        depth_or_array_layers: 1,
    };
    let bind_group_layout_entries = vec![wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::StorageTexture {
            access: wgpu::StorageTextureAccess::Atomic,
            format,
            view_dimension: wgpu::TextureViewDimension::D2,
        },
        count: None,
    }];

    let bind_group_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &bind_group_layout_entries,
        });

    let pipeline_layout = ctx
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
    let shader = ctx.device.create_shader_module(desc);
    let pipeline = ctx
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("image atomics pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("cs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

    let tex = ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        dimension: wgpu::TextureDimension::D2,
        size,
        format,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::STORAGE_ATOMIC
            | wgpu::TextureUsages::COPY_SRC,
        mip_level_count: 1,
        sample_count: 1,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor {
        format: Some(format),
        aspect: wgpu::TextureAspect::All,
        ..wgpu::TextureViewDescriptor::default()
    });
    let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&view),
        }],
    });

    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    let mut rpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: None,
        timestamp_writes: None,
    });
    rpass.set_pipeline(&pipeline);
    rpass.set_bind_group(0, Some(&bind_group), &[]);
    rpass.dispatch_workgroups(size.width, size.height, 1);
    drop(rpass);
    ctx.queue.submit(Some(encoder.finish()));

    let read_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (size.height * size.width * size.depth_or_array_layers * pixel_bytes) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &read_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width * pixel_bytes),
                rows_per_image: Some(size.height),
            },
        },
        size,
    );

    ctx.queue.submit(Some(encoder.finish()));

    let slice = read_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| ());
    ctx.async_poll(wgpu::Maintain::wait())
        .await
        .panic_on_timeout();
    let data: Vec<u8> = slice.get_mapped_range().to_vec();

    assert_eq!(data.len() as u32, size.width * size.height * pixel_bytes);
    for (i, long) in data.chunks(pixel_bytes as usize).enumerate() {
        let x = (i as u32 % size.width) as u8;
        let y = (i as u32 / size.width) as u8;
        assert_eq!(long[0], u8::min(x, y), "{i}");
        assert_eq!(
            long[1..pixel_bytes as usize],
            [0].repeat(pixel_bytes as usize - 1)
        );
    }
}
