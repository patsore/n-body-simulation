
use wgpu::{Buffer, BufferUsages, Device};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],

    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<Vertex>()) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Circle {
    pub world_pos: [f32; 3],
    pub radius: f32,
    pub color: u32,
}

impl Circle {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Uint32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<Circle>()) as wgpu::BufferAddress,
            //TODO maybe not
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

const CIRCLES: &[Circle] = &[
    Circle {
        world_pos: [0.0, -5.0, 0.0],
        radius: 0.1,
        color: 0xFF0000FF,
    },
    Circle {
        world_pos: [5.0, 0.0, 0.0],
        radius: 0.1,
        color: 0x00FF00FF,
    },
    Circle {
        world_pos: [0.0, 5.0, 0.0],
        radius: 0.1,
        color: 0x0000FFFF,
    },
    Circle {
        world_pos: [-5.0, 0.0, 0.0],
        radius: 0.1,
        color: 0xFF00FFFF,
    },
    Circle {
        world_pos: [3.5355339059, 3.5355339059, 0.0],  // 45-degree angle
        radius: 0.1,
        color: 0x00FFFFFF,
    },
];

fn pack_rgba_into_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let result = (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | a as u32;
    result
}

pub fn initialize_circle_and_vertex_bufs(device: &Device) -> (Vec<Circle>, Buffer) {
    // let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
    //     label: None,
    //     contents: bytemuck::cast_slice(VERTICES),
    //     usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    // });

    // let vertex_len = VERTICES.len();

    // return (vertex_buffer, vertex_len as u32);

    let circles = CIRCLES.to_vec();

    let circle_buffer = device.create_buffer_init(&BufferInitDescriptor{
       label: Some("Circle Buffer"),
        contents: bytemuck::cast_slice(CIRCLES),
        usage: BufferUsages::VERTEX,
    });

    return (circles, circle_buffer);
}