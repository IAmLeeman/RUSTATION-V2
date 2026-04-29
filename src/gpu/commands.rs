enum GP0Command {
    FlatTriangle {
        color: [u8; 3],
        vo: Vertex,
        v1: Vertex,
        v2: Vertex,
    },
}