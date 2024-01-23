use bytemuck::{Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{core::mesh::{Mesh, Vertex}, Context};

#[repr(C)]
#[derive(Debug,Clone, Copy,Pod, Zeroable)]
struct LineVertex {
    position:[f32;2]
}
const LINE_LAYOUT: wgpu::VertexBufferLayout  = wgpu::VertexBufferLayout{
    array_stride: std::mem::size_of::<LineVertex>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2]
};
impl Vertex for LineVertex {}

pub struct LineRenderer {
    pipeline: wgpu::RenderPipeline,
    queued: Vec<(f32, f32, f32, f32)>
}

impl LineRenderer {
    pub fn new(ctx: &Context) -> LineRenderer {

        let shader = ctx.wgpu_device().create_shader_module(wgpu::include_wgsl!("shaders/line.wgsl"));

        let layout = ctx.wgpu_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Line Renderer Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        let pipeline = ctx.wgpu_device().create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("Line Renderer Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    LINE_LAYOUT
                ]
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[
                    Some(
                        wgpu::ColorTargetState {
                            format: *ctx.wgpu_surface_format(),
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL
                        }
                    )
                ]
            }),
            multiview: None
        });

        LineRenderer { pipeline, queued: vec![] }
    }
    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1:f32) {
        self.queued.push((x0,y0,x1,y1))
    }
    pub fn render(&mut self, ctx: &Context) {
        let output = ctx.wgpu_surface().get_current_texture().unwrap();

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = ctx.wgpu_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut vertices = vec![];
        let mut indices = vec![];

        let mut offset = 0;

        for line in &self.queued {
            vertices.push(LineVertex { position: [line.0, line.1] });
            vertices.push(LineVertex { position: [line.2, line.3] });
            indices.push(0 + offset);
            indices.push(1 + offset);

            offset += 2;
        }
    
        let mesh: Mesh<LineVertex> = Mesh::new(ctx, &vertices, &indices);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            render_pass.set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }
    
        ctx.wgpu_queue().submit(std::iter::once(encoder.finish()));
        output.present();

        self.queued.clear()
    }
}
