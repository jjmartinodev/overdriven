use std::marker::PhantomData;

use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::Context;

pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {}

pub struct Mesh<V:Vertex> {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    phantom: PhantomData<V>
}

impl<V:Vertex> Mesh<V> {
    pub fn new(ctx: &Context, vertices: &[V], indices: &[u32]) -> Mesh<V> {
        let vertex_buffer = ctx.wgpu_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX
        });
        let index_buffer = ctx.wgpu_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX
        });
        Mesh {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            phantom: PhantomData
        }
    }
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }
    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }
}