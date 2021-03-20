use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use num_traits::Float;

pub fn wrap<F: Float>(val: F, mut from: F, mut to: F) -> F {
    if from > to {
        std::mem::swap(&mut from, &mut to);
    }
    let cycle = to - from;
    if cycle == F::zero() {
        return to;
    }
    val - cycle * ((val - from) / cycle).floor()
}

pub fn clamp<T: PartialOrd>(input: T, mut min: T, mut max: T) -> T {
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub trait SpawnLine {
    fn spawn_line(
        &mut self,
        material: Handle<ColorMaterial>,
        line: (Vec2, Vec2),
        width: f32,
    ) -> &mut Self;
}

impl SpawnLine for Commands {
    fn spawn_line(
        &mut self,
        material: Handle<ColorMaterial>,
        line: (Vec2, Vec2),
        width: f32,
    ) -> &mut Self {
        self.spawn(GeometryBuilder::build_as(
            &shapes::Line(line.0, line.1),
            material,
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(width)),
            Transform::default(),
        ))
    }
}
