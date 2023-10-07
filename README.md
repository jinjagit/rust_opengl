Set up basic Rust framework for using OpenGL.

Based on [Understanding OpenGL basics in Rust](https://blog.logrocket.com/understanding-opengl-basics-rust/), by Rosario De Chiara.

I have modularised the code to use 3 single-method modules:
- `shape::shape` - defines vertices used as a basic mesh
- `vertex_shader::vertex_shader_src` - returns the string used to define the shader
- `fragment_shader::fragment_shader_src` - returns the string used to define the shader