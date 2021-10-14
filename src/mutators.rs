pub trait VertexMutator<I> {
    type Vertex;
    fn add_vertex(&mut self) -> Option<&mut Self::Vertex>;
    fn remove_vertex(&mut self) -> Option<Self::Vertex>;
    fn clear_vertices(&mut self);
}

pub trait EdgeMutator<I> {
    type Edge;
    fn add_edge(&mut self, source: I, target: I) -> Option<&mut Self::Edge>;
    fn remove_edge(&mut self) -> Option<Self::Edge>;
    fn clear_edges(&mut self);
}
