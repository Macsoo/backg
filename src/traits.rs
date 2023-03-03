pub trait Colored {
    fn recolor(&mut self, red: u8, green: u8, blue: u8);
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
        todo!()
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
    fn set_visibility(&self, visible: bool);
    fn get_visibility(&self) -> bool;
}

pub trait Group {

}

pub trait Partition {

}

pub trait Object: Positioned + Rotated + Scaled + Visible {

}

pub trait Meshed: Group + Partition {

}

pub trait Drawable: Object + Meshed {

}