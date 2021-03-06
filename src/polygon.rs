//! Draw polygon

use internal;
use triangulation;
use Graphics;
use Color;
use DrawState;
use vecmath::Matrix2d;

/// A polygon
#[derive(Copy, Clone)]
pub struct Polygon {
    /// The color of the polygon
    pub color: internal::Color,
}

impl Polygon {
    /// Creates new polygon
    pub fn new(color: internal::Color) -> Polygon {
        Polygon {
            color: color,
        }
    }

    /// Draws polygon
    pub fn draw<G>(
        &self,
        polygon: internal::Polygon,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        g.tri_list(
            draw_state,
            &self.color,
            |f|
        triangulation::with_polygon_tri_list(
            transform,
            polygon,
            |vertices| f(vertices)
        ));
    }

    /// Draws tweened polygon with linear interpolation
    pub fn draw_tween_lerp<G>(
        &self,
        polygons: internal::Polygons,
        tween_factor: internal::Scalar,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        if self.color[3] == 0.0 { return; }
        g.tri_list(
            draw_state,
            &self.color,
            |f|
        triangulation::with_lerp_polygons_tri_list(
            transform,
            polygons,
            tween_factor,
            |vertices| f(vertices)
        ));
    }
}

quack! {
    p: Polygon[]
    get:
        fn () -> Color [] { Color(p.color) }
    set:
        fn (val: Color) [] { p.color = val.0 }
    action:
}

#[cfg(test)]
mod test {
    use quack::Set;
    use super::Polygon;
    use Color;

    #[test]
    fn test_polygon() {
        let _polygon = Polygon::new([1.0; 4])
            .set(Color([0.0; 4]));
    }
}
