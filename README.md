Set up basic Rust framework for using OpenGL.

Based on [Understanding OpenGL basics in Rust](https://blog.logrocket.com/understanding-opengl-basics-rust/), by Rosario De Chiara.

I have modularised the code to use 3 single-method modules:
- `shape::shape` - defines vertices used as a basic mesh
- `vertex_shader::vertex_shader_src` - returns the string used to define the shader
- `fragment_shader::fragment_shader_src` - returns the string used to define the shader

<img width="2560" alt="Screenshot 2023-10-07 at 20 07 25" src="https://github.com/jinjagit/rust_opengl/assets/3944042/2aac502d-ba3c-4a5e-a504-d93ea877b952">

## Notes

Can run code conditional on OS: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
Can get current display resolution on MacOs using screenresolution (if installed) https://apple.stackexchange.com/questions/449891/a-single-keyboard-shortcut-that-toggles-between-2-resolutions-on-macos-12-6#:~:text=1-,Save%20this%20answer.,-The%20command%20line