pub trait Colored {
    fn set_color(&mut self, red: u8, green: u8, blue: u8);
    fn get_color(&self) -> (u8, u8, u8);
}

pub trait Shadowed {
    fn light_source<P: Positioned>(&mut self, position: P);
}

pub trait Positioned {
    fn set_position(&mut self, x: f32, y: f32, z: f32);
    fn get_position(&self) -> (f32, f32, f32);
    fn translate_by(&mut self, dx: f32, dy: f32, dz: f32) {
        let (x, y, z) = self.get_position();
        self.set_position(x + dx, y + dy, z + dz)
    }
}

pub trait Rotated {
    fn set_rotation(&mut self, x: f32, y: f32, z: f32);
    fn get_rotation(&self) -> (f32, f32, f32);
    fn rotate_around(&mut self, degree: f32, vector: (f32, f32, f32)) {

    }
}

pub trait Scaled {
    fn set_scale(&mut self, x: f32, y: f32, z: f32);
    fn get_scale(&self) -> (f32, f32, f32);
    fn scale_by(&mut self, dx: f32, dy: f32, dz: f32) {
        let (x, y, z) = self.get_scale();
        self.set_scale(x + dx, y + dy, z + dz)
    }
}

pub trait Visible {
    fn set_visibility(&mut self, visible: bool);
    fn get_visibility(&self) -> bool;
}

pub trait Transform {
    fn get_properties(&self) -> ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), bool);
    fn set_properties(&mut self, properties: ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), bool));
}

impl<T: Transform> Positioned for T {
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        let mut prop = self.get_properties();
        prop.0 = (x, y, z);
        self.set_properties(prop);
    }
    fn get_position(&self) -> (f32, f32, f32) {
        let (pos, ..) = self.get_properties();
        pos
    }
}

impl<T: Transform> Rotated for T {
    fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        let mut prop = self.get_properties();
        prop.1 = (x, y, z);
        self.set_properties(prop);
    }
    fn get_rotation(&self) -> (f32, f32, f32) {
        let (_, rot, ..) = self.get_properties();
        rot
    }
}

impl<T: Transform> Scaled for T {
    fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        let mut prop = self.get_properties();
        prop.2 = (x, y, z);
        self.set_properties(prop);
    }
    fn get_scale(&self) -> (f32, f32, f32) {
        let (.., sca, _) = self.get_properties();
        sca
    }
}

impl<T: Transform> Visible for T {
    fn set_visibility(&mut self, visibility: bool) {
        let mut prop = self.get_properties();
        prop.3 = visibility;
        self.set_properties(prop);
    }
    fn get_visibility(&self) -> bool {
        let (.., vis) = self.get_properties();
        vis
    }
}

pub trait Meshed {
    fn get_vertices(&self) -> &[f32];
    fn set_vertices(&mut self, vertices: &[f32]);
    fn get_indices(&self) -> &[i32];
    fn set_indices(&mut self, indices: &[i32]);
}

macro_rules! object {
    ($name:ident { $($id:ident: $ty:ty),* }) => {
        pub struct $name {
            vertices: Vec<f32>,
            indices: Vec<i32>,
            position: (f32, f32, f32),
            rotation: (f32, f32, f32),
            scale: (f32, f32, f32),
            visible: bool,
            $(
            $id: $ty,
            )*
        }

        impl Transform for $name {
            fn get_properties(&self) -> ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), bool) {
                (self.position, self.rotation, self.scale, self.visible)
            }
            fn set_properties(&mut self, properties: ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), bool)) {
                (self.position, self.rotation, self.scale, self.visible) = properties;
            }
        }

        impl Meshed for $name {
            fn get_vertices(&self) -> &[f32] {
                &self.vertices[..]
            }
            fn set_vertices(&mut self, vertices: &[f32]) {
                self.vertices = vertices.to_vec();
            }
            fn get_indices(&self) -> &[i32] {
                &self.indices[..]
            }
            fn set_indices(&mut self, indices: &[i32]) {
                self.indices = indices.to_vec();
            }
        }

        impl $name {
            pub fn empty($($id: $ty)*) -> Self {
                let position = (0.0, 0.0, 0.0);
                let rotation = (0.0, 0.0, 0.0);
                let scale = (1.0, 1.0, 1.0);
                let visible = true;
                let vertices = Vec::new();
                let indices = Vec::new();
                Self {
                    position,
                    rotation,
                    scale,
                    visible,
                    vertices,
                    indices,
                    $($id,)*
                }
            }
        }
    };
}

object!(Cube { });

impl Cube {
    pub fn new() -> Self {
        let mut cube = Cube::empty();
        cube.set_vertices(vec![
            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
        ].as_slice());

        cube
    }
}