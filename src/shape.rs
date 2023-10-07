#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

pub fn shape() -> Vec<Vertex> {
  implement_vertex!(Vertex, position);


  let vertex1 = Vertex { position: [0.0, 0.0] };
  let vertex2 = Vertex { position: [0.0, 1.0] };
  let vertex3 = Vertex { position: [1.0, 1.0] };
  let vertex4 = Vertex { position: [1.0, 0.0] };
  
  vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4]
}
