#[vertex_format]
#[derive(Copy, Clone)]
pub struct Vertex {
    #[name = "position"]
    pub pos: [f32; 3],

//     #[name = "color"]
//     rgb: [f32; 3],
}

pub static VERTEX: &'static [u8] = b"
    #version 150 core
    uniform mat4 mvp;

    void main() {
        gl_Position = mvp * position;
    }
";

pub static FRAGMENT: &'static [u8] = b"
    #version 150 core
    out vec3 fragment;

    void main() {
        fragment = vec3(0.2);
    }
";
