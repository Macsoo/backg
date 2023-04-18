use std::collections::HashMap;
use crate::object;

object!(Sphere("shaders/galaxy.vert", "shaders/galaxy.frag") {

});

impl Sphere {
    pub fn generate_icosahedron() -> Vec<f32> {
        use std::f32::consts::PI;
        let mut vertices: Vec<f32> = std::iter::repeat(0.0).take(12 * 3).collect();

        let horizontal_angle = PI / 180.0 * 72.0;
        let vertical_angle = (1.0f32 / 2.0f32).atan();
        let (mut i1, mut i2);
        let (z, xy) = (vertical_angle.sin(), vertical_angle.cos());
        let mut h_angle_1 = -PI / 2.0 - horizontal_angle / 2.0;
        let mut h_angle_2 = -PI / 2.0;

        *vertices.get_mut(0).unwrap() = 0.0;
        *vertices.get_mut(1).unwrap() = 0.0;
        *vertices.get_mut(2).unwrap() = 1.0;

        for i in 1..=5 {
            i1 = i * 3;
            i2 = (i + 5) * 3;

            let h1_cos = h_angle_1.cos();
            let h1_sin = h_angle_1.sin();
            let h2_cos = h_angle_2.cos();
            let h2_sin = h_angle_2.sin();

            *vertices.get_mut(i1 + 0).unwrap() = xy * h1_cos;
            *vertices.get_mut(i2 + 0).unwrap() = xy * h2_cos;
            *vertices.get_mut(i1 + 1).unwrap() = xy * h1_sin;
            *vertices.get_mut(i2 + 1).unwrap() = xy * h2_sin;
            *vertices.get_mut(i1 + 2).unwrap() = z;
            *vertices.get_mut(i2 + 2).unwrap() = -z;

            h_angle_1 += horizontal_angle;
            h_angle_2 += horizontal_angle;
        }

        i1 = 11 * 3;

        *vertices.get_mut(i1 + 0).unwrap() = 0.0;
        *vertices.get_mut(i1 + 1).unwrap() = 0.0;
        *vertices.get_mut(i1 + 2).unwrap() = -1.0;

        vertices
    }

    pub fn new(subdivision: usize) -> Self {
        let mut sphere = Sphere::empty();
        sphere.vertices = Sphere::generate_icosahedron();
        sphere.indices = vec![
            0, 1, 2,
            0, 2, 3,
            0, 3, 4,
            0, 4, 5,
            0, 5, 1,
            1, 6, 2,
            2, 6, 7,
            2, 7, 3,
            3, 7, 8,
            3, 8, 4,
            4, 8, 9,
            4, 9, 5,
            5, 9, 10,
            5, 10, 1,
            1, 10, 6,
            11, 7, 6,
            11, 8, 7,
            11, 9, 8,
            11, 10, 9,
            11, 6, 10,
        ];
        for _ in 0..subdivision {
            let mut new_index = sphere.vertices.len() as u32 / 3;
            let mut middles = HashMap::<(u32, u32), u32>::new();
            let mut temp_indices = Vec::new();
            for (_ind, next) in sphere.indices.chunks(3).enumerate() {
                let (f, s, t) = (next[0], next[1], next[2]);
                let middle_fs = {
                    if let Some(m) = middles.get(&(f, s)) {
                        Some(m)
                    } else if let Some(m) = middles.get(&(s, f)) {
                        Some(m)
                    } else {
                        None
                    }
                };
                let fs = {
                    if let Some(&m) = middle_fs {
                        m
                    } else {
                        let x = (*sphere.vertices.get(f as usize * 3 + 0).unwrap() + *sphere.vertices.get(s as usize * 3 + 0).unwrap()) / 2.0;
                        let y = (*sphere.vertices.get(f as usize * 3 + 1).unwrap() + *sphere.vertices.get(s as usize * 3 + 1).unwrap()) / 2.0;
                        let z = (*sphere.vertices.get(f as usize * 3 + 2).unwrap() + *sphere.vertices.get(s as usize * 3 + 2).unwrap()) / 2.0;
                        let len = (x * x + y * y + z * z).sqrt();
                        let x = x / len;
                        let y = y / len;
                        let z = z / len;
                        sphere.vertices.push(x);
                        sphere.vertices.push(y);
                        sphere.vertices.push(z);
                        middles.insert((f, s), new_index);
                        let i = new_index;
                        new_index += 1;
                        i
                    }
                };
                let middle_st = {
                    if let Some(m) = middles.get(&(s, t)) {
                        Some(m)
                    } else if let Some(m) = middles.get(&(t, s)) {
                        Some(m)
                    } else {
                        None
                    }
                };
                let st = {
                    if let Some(&m) = middle_st {
                        m
                    } else {
                        let x = (*sphere.vertices.get(s as usize * 3 + 0).unwrap() + *sphere.vertices.get(t as usize * 3 + 0).unwrap()) / 2.0;
                        let y = (*sphere.vertices.get(s as usize * 3 + 1).unwrap() + *sphere.vertices.get(t as usize * 3 + 1).unwrap()) / 2.0;
                        let z = (*sphere.vertices.get(s as usize * 3 + 2).unwrap() + *sphere.vertices.get(t as usize * 3 + 2).unwrap()) / 2.0;
                        let len = (x * x + y * y + z * z).sqrt();
                        let x = x / len;
                        let y = y / len;
                        let z = z / len;
                        sphere.vertices.push(x);
                        sphere.vertices.push(y);
                        sphere.vertices.push(z);
                        middles.insert((s, t), new_index);
                        let i = new_index;
                        new_index += 1;
                        i
                    }
                };
                let middle_ft = {
                    if let Some(m) = middles.get(&(f, t)) {
                        Some(m)
                    } else if let Some(m) = middles.get(&(t, f)) {
                        Some(m)
                    } else {
                        None
                    }
                };
                let ft = {
                    if let Some(&m) = middle_ft {
                        m
                    } else {
                        let x = (*sphere.vertices.get(f as usize * 3 + 0).unwrap() + *sphere.vertices.get(t as usize * 3 + 0).unwrap()) / 2.0;
                        let y = (*sphere.vertices.get(f as usize * 3 + 1).unwrap() + *sphere.vertices.get(t as usize * 3 + 1).unwrap()) / 2.0;
                        let z = (*sphere.vertices.get(f as usize * 3 + 2).unwrap() + *sphere.vertices.get(t as usize * 3 + 2).unwrap()) / 2.0;
                        let len = (x * x + y * y + z * z).sqrt();
                        let x = x / len;
                        let y = y / len;
                        let z = z / len;
                        sphere.vertices.push(x);
                        sphere.vertices.push(y);
                        sphere.vertices.push(z);
                        middles.insert((f, t), new_index);
                        let i = new_index;
                        new_index += 1;
                        i
                    }
                };
                temp_indices.push(f);
                temp_indices.push(fs);
                temp_indices.push(ft);
                temp_indices.push(fs);
                temp_indices.push(st);
                temp_indices.push(ft);
                temp_indices.push(fs);
                temp_indices.push(s);
                temp_indices.push(st);
                temp_indices.push(ft);
                temp_indices.push(st);
                temp_indices.push(t);
            }
            sphere.indices = temp_indices;
        }
        sphere.calculate_normals();
        sphere
    }
}