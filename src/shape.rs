#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

pub fn shape() -> Vec<Vertex> {
    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [-0.5, 0.5] };
    let vertex3 = Vertex { position: [0.5, 0.5] };
    let vertex4 = Vertex { position: [0.5, -0.5] };
    
    vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4]
}
