pub fn vertex_shader_src() -> String {
    format!(r#"{}"#,
        "
            #version 140
            in vec2 position;
            uniform mat4 matrix;
            out vec2 my_attr;      

            void main() {
                my_attr = position + vec2(0.5, 0.5); 
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "
    )
}
