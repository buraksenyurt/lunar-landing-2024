use crate::viper::{Dimension, Rectangle, Vector};
use sdl2::rect::Point;

pub struct Math;

impl Math {
    pub fn get_index(x: u32, y: u32, dimension: Dimension, col_count: u32) -> u32 {
        (y / dimension.height) * col_count + (x / dimension.width)
    }

    pub fn get_position(index: u32, dimension: Dimension, col_count: u32) -> (u32, u32) {
        (
            (index % col_count) * dimension.width,
            (index / col_count) * dimension.height,
        )
    }

    pub fn get_unit_vector(a: Vector, b: Vector) -> Vector {
        let vec_ab = (b.x - a.x, b.y - a.y);
        let magnitude_ab = ((vec_ab.0).powf(2.0) + (vec_ab.1).powf(2.0)).sqrt();
        Vector::new(vec_ab.0 / magnitude_ab, vec_ab.1 / magnitude_ab)
    }

    pub fn find_ground_height(points: &[Point], x: i32) -> i32 {
        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            if x >= p1.x && x <= p2.x {
                return Math::interpolate_height(&[p1, p2], x);
            }
        }
        0
    }

    fn interpolate_height(points: &[Point], x: i32) -> i32 {
        if x <= points.first().unwrap().x {
            return points.first().unwrap().y;
        }
        if x >= points.last().unwrap().x {
            return points.last().unwrap().y;
        }

        for window in points.windows(2) {
            let (p1, p2) = (window[0], window[1]);
            if x >= p1.x && x <= p2.x {
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                return p1.y + (x - p1.x) * dy / dx;
            }
        }
        points.first().unwrap().y
    }

    // Axis-Aligned Bounding Box AABB hesaplama tekniğine göre
    pub fn check_collision(rectangle_1: Rectangle, rectangle_2: Rectangle) -> bool {
        let rect1_right = rectangle_1.location.x + rectangle_1.dimension.width as i32;
        let rect1_bottom = rectangle_1.location.y + rectangle_1.dimension.height as i32;
        let rect2_right = rectangle_2.location.x + rectangle_2.dimension.width as i32;
        let rect2_bottom = rectangle_2.location.y + rectangle_2.dimension.height as i32;

        rectangle_1.location.x < rect2_right
            && rect1_right > rectangle_2.location.x
            && rectangle_1.location.y < rect2_bottom
            && rect1_bottom > rectangle_2.location.y
    }
}
