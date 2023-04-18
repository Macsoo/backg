use std::ffi::CString;
use crate::gl;
use crate::math::{Camera, Mat4x4, Vec3};

pub trait Colored {
    fn set_color(&mut self, red: u8, green: u8, blue: u8);
    fn get_color(&self) -> (u8, u8, u8);
}

pub trait Shadowed {
    fn light_source<P: Positioned>(&mut self, position: P);
}

pub trait Positioned {
    fn translate_by(&mut self, v: Vec3);
}

pub trait Rotated {
    fn rotate_around(&mut self, d: f32, v: Vec3);
}

pub trait Scaled {
    fn scale_by(&mut self, v: Vec3);
    fn scaled_by(&mut self, d: f32);
}

pub trait Visible {
    fn set_visibility(&mut self, visible: bool);
    fn get_visibility(&self) -> bool;
}

pub trait Transform {
    fn get_matrix(&self) -> Mat4x4;
    fn set_matrix(&mut self, matrix: Mat4x4);
}

impl<T: Transform> Positioned for T {
    fn translate_by(&mut self, v: Vec3) {
        let mut m = self.get_matrix();
        m.translate(v);
        self.set_matrix(m);
    }
}

impl<T: Transform> Rotated for T {
    fn rotate_around(&mut self, d: f32, v: Vec3) {
        let mut m = self.get_matrix();
        m.rotate(d, v);
        self.set_matrix(m);
    }
}

impl<T: Transform> Scaled for T {
    fn scale_by(&mut self, v: Vec3) {
        let mut m = self.get_matrix();
        m.scale(v);
        self.set_matrix(m);
    }

    fn scaled_by(&mut self, d: f32) {
        let mut m = self.get_matrix();
        m.scale(Vec3(d, d, d));
        self.set_matrix(m);
    }
}

pub trait Meshed {
    fn get_vertices(&self) -> &[f32];
    fn set_vertices(&mut self, vertices: &[f32]);
    fn get_indices(&self) -> &[u32];
    fn set_indices(&mut self, indices: &[u32]);
    fn get_normals(&self) -> &[f32];
    fn set_normals(&mut self, normals: &[f32]);
}

pub trait Drawable {
    fn draw(&self, camera: &Camera);
}

pub trait Shaded {
    fn get_shader_program(&self) -> u32;
    fn set_shader_program(&mut self, id: u32);
    fn compile_shaders(&mut self, vertex: Option<&str>, fragment: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            let common_vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            let common_fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);

            let vertex_shader_code = match vertex {
                Some(path) => std::fs::read_to_string(path)?,
                None => String::from("#version 460 core
layout(location = 0) in vec3 vPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(vPos, 1.0);
}")
            };

            let fragment_shader_code = match fragment {
                Some(path) => std::fs::read_to_string(path)?,
                None => String::from("#version 460 core
out vec3 color;

void main() {
    color = vec3(0.2, 0.7, 0.9);
}")
            };

            let cstring = CString::new(vertex_shader_code)?;
            let pointer = cstring.as_ptr();
            gl::ShaderSource(vertex_shader_id, 1, &pointer, core::ptr::null());
            gl::CompileShader(vertex_shader_id);

            let mut result = 0;
            let mut info_log_length = 0;
            gl::GetShaderiv(vertex_shader_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(vertex_shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            if info_log_length > 0 {
                let mut vec = Vec::with_capacity(info_log_length as usize + 1);
                vec.extend([b' '].iter().cycle().take(info_log_length as usize));
                let cs = CString::from_vec_with_nul_unchecked(vec);
                gl::GetShaderInfoLog(vertex_shader_id, info_log_length, core::ptr::null_mut(), cs.as_ptr() as *mut gl::types::GLchar);
                println!("{}", cs.to_str()?);
            }

            let cstring = CString::new(std::fs::read_to_string("shaders/common.vert").unwrap())?;
            let pointer = cstring.as_ptr();
            gl::ShaderSource(common_vertex_shader_id, 1, &pointer, core::ptr::null());
            gl::CompileShader(common_vertex_shader_id);

            let mut result = 0;
            let mut info_log_length = 0;
            gl::GetShaderiv(common_vertex_shader_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(common_vertex_shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            if info_log_length > 0 {
                let mut vec = Vec::with_capacity(info_log_length as usize + 1);
                vec.extend([b' '].iter().cycle().take(info_log_length as usize));
                let cs = CString::from_vec_with_nul_unchecked(vec);
                gl::GetShaderInfoLog(common_vertex_shader_id, info_log_length, core::ptr::null_mut(), cs.as_ptr() as *mut gl::types::GLchar);
                println!("{}", cs.to_str()?);
            }

            let cstring = CString::new(fragment_shader_code)?;
            let pointer = cstring.as_ptr();
            gl::ShaderSource(fragment_shader_id, 1, &pointer, core::ptr::null());
            gl::CompileShader(fragment_shader_id);

            let mut result = 0;
            let mut info_log_length = 0;
            gl::GetShaderiv(fragment_shader_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(fragment_shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            if info_log_length > 0 {
                let mut vec = Vec::with_capacity(info_log_length as usize + 1);
                vec.extend([b' '].iter().cycle().take(info_log_length as usize));
                let cs = CString::from_vec_with_nul_unchecked(vec);
                gl::GetShaderInfoLog(fragment_shader_id, info_log_length, core::ptr::null_mut(), cs.as_ptr() as *mut gl::types::GLchar);
                println!("{}", cs.to_str()?);
            }

            let cstring = CString::new(std::fs::read_to_string("shaders/common.frag").unwrap())?;
            let pointer = cstring.as_ptr();
            gl::ShaderSource(common_fragment_shader_id, 1, &pointer, core::ptr::null());
            gl::CompileShader(common_fragment_shader_id);

            let mut result = 0;
            let mut info_log_length = 0;
            gl::GetShaderiv(common_fragment_shader_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(common_fragment_shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            if info_log_length > 0 {
                let mut vec = Vec::with_capacity(info_log_length as usize + 1);
                vec.extend([b' '].iter().cycle().take(info_log_length as usize));
                let cs = CString::from_vec_with_nul_unchecked(vec);
                gl::GetShaderInfoLog(common_fragment_shader_id, info_log_length, core::ptr::null_mut(), cs.as_ptr() as *mut gl::types::GLchar);
                println!("{}", cs.to_str()?);
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, common_vertex_shader_id);
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, common_fragment_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);

            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut result);
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            if info_log_length > 0 {
                let mut vec = Vec::with_capacity(info_log_length as usize + 1);
                vec.extend([b' '].iter().cycle().take(info_log_length as usize));
                let cs = CString::from_vec_with_nul_unchecked(vec);
                gl::GetProgramInfoLog(program_id, info_log_length, core::ptr::null_mut(), cs.as_ptr() as *mut gl::types::GLchar);
                println!("{}", cs.to_str()?);
            }

            gl::DetachShader(program_id, common_vertex_shader_id);
            gl::DetachShader(program_id, vertex_shader_id);
            gl::DetachShader(program_id, common_fragment_shader_id);
            gl::DetachShader(program_id, fragment_shader_id);

            gl::DeleteShader(common_vertex_shader_id);
            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(common_fragment_shader_id);
            gl::DeleteShader(fragment_shader_id);

            self.set_shader_program(program_id);

            Ok(())
        }
    }
}

impl<T: Transform + Visible + Meshed + Shaded> Drawable for T {
    fn draw(&self, camera: &Camera) {
        unsafe {
            if !self.get_visibility() { return; }
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut array_buffer = 0;
            gl::GenBuffers(1, &mut array_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, array_buffer);
            let vertices = self.get_vertices();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            let mut element_array_buffer = 0;
            gl::GenBuffers(1, &mut element_array_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_array_buffer);
            let indices = self.get_indices();
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(indices) as _,
                indices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as _,
                0 as _,
            );
            gl::EnableVertexAttribArray(0);

            let mut normal_buffer = 0;
            gl::GenBuffers(1, &mut normal_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
            let vertices = self.get_normals();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as _,
                0 as _,
            );
            gl::EnableVertexAttribArray(1);

            gl::UseProgram(self.get_shader_program());

            let cstring = CString::new("model").unwrap();
            let loc = gl::GetUniformLocation(self.get_shader_program(), cstring.as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::TRUE, self.get_matrix().0.as_ptr());

            let cstring = CString::new("view").unwrap();
            let loc = gl::GetUniformLocation(self.get_shader_program(), cstring.as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::TRUE, camera.view.0.as_ptr());

            let cstring = CString::new("projection").unwrap();
            let loc = gl::GetUniformLocation(self.get_shader_program(), cstring.as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::TRUE, camera.projection.0.as_ptr());

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as _,
                gl::UNSIGNED_INT,
                0 as _,
            );

            gl::DeleteBuffers(1, &array_buffer);
            gl::DeleteBuffers(1, &element_array_buffer);
            gl::DeleteBuffers(1, &normal_buffer);

            gl::UseProgram(0);
        }
    }
}

#[macro_export]
macro_rules! object {
    ($name:ident($v:expr, $f:expr) { $($id:ident: $ty:ty),* }) => {
        use crate::math::*;
        use crate::traits::*;
        pub struct $name {
            vertices: Vec<f32>,
            indices: Vec<u32>,
            matrix: Mat4x4,
            visible: bool,
            program: u32,
            normals: Vec<f32>,
            $(
            $id: $ty,
            )*
        }

        impl Visible for $name {
            fn set_visibility(&mut self, visible: bool) {
                self.visible = visible;
            }
            fn get_visibility(&self) -> bool {
                self.visible
            }
        }

        impl Transform for $name {
            fn get_matrix(&self) -> Mat4x4 {
                self.matrix.clone()
            }
            fn set_matrix(&mut self, matrix: Mat4x4) {
                self.matrix = matrix;
            }
        }

        impl Meshed for $name {
            fn get_vertices(&self) -> &[f32] {
                &self.vertices[..]
            }
            fn set_vertices(&mut self, vertices: &[f32]) {
                self.vertices = vertices.to_vec();
            }
            fn get_indices(&self) -> &[u32] {
                &self.indices[..]
            }
            fn set_indices(&mut self, indices: &[u32]) {
                self.indices = indices.to_vec();
            }
            fn get_normals(&self) -> &[f32] {
                &self.normals[..]
            }
            fn set_normals(&mut self, normals: &[f32]) {
                self.normals = normals.to_vec();
            }
        }

        impl Shaded for $name {
            fn get_shader_program(&self) -> u32 {
                self.program
            }
            fn set_shader_program(&mut self, id: u32) {
                self.program = id;
            }
        }

        impl $name {
            pub fn empty($($id: $ty)*) -> Self {
                let matrix = Mat4x4::identity();
                let visible = true;
                let vertices = Vec::new();
                let indices = Vec::new();
                let normals = Vec::new();
                let program = 0;
                let mut sself = Self {
                    matrix,
                    visible,
                    vertices,
                    indices,
                    program,
                    normals,
                    $($id,)*
                };
                sself.compile_shaders(Some($v), Some($f)).unwrap();
                sself
            }

            pub fn calculate_normals(&mut self) {
                self.normals = vec![0.0; self.vertices.len()];
                let mut chunks = self.indices.chunks(3);
                while let Some([f, s, t]) = chunks.next() {
                    let mut vec3s = [Vec3::zero(); 3];
                    for (ind, i) in [f, s, t].iter().enumerate() {
                        vec3s[ind] = Vec3(
                            *self.vertices.get(**i as usize * 3 + 0).unwrap(),
                            *self.vertices.get(**i as usize * 3 + 1).unwrap(),
                            *self.vertices.get(**i as usize * 3 + 2).unwrap(),
                        )
                    }
                    let face_normal = vec3s.into_iter().sum::<Vec3>() / 3.0;
                    for i in [f, s, t] {
                        *self.normals.get_mut(*i as usize * 3 + 0).unwrap() += face_normal.x();
                        *self.normals.get_mut(*i as usize * 3 + 1).unwrap() += face_normal.y();
                        *self.normals.get_mut(*i as usize * 3 + 2).unwrap() += face_normal.z();
                    }
                }
                let chunks = self.normals.chunks_exact_mut(3);
                chunks.for_each(|l| {
                    if let [f, s, t] = l {
                        let v = Vec3(*f, *s, *t).normalized();
                        (*f, *s, *t) = (v.0, v.1, v.2);
                    } else { unreachable!() }
                });
            }
        }
    };
}