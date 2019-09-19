use crate::math::{ Vec2, Vec3, Vec4, Quat, Transform };

use minterpolate::{ InterpolationFunction, InterpolationPrimitive };

/// A generic object which contains a property of type T which is sequenced over time.
pub trait Sequenced<T>: Send + Sync {
    fn sample_at(&self, t: f32) -> T;
}

macro_rules! impl_inherent_sequenced {
    ($($type:ty,)*) => {
        $(impl Sequenced<$type> for $type {
            fn sample_at(&self, _t: f32) -> Self {
                self.clone()
            }
        })*
    }
}

impl_inherent_sequenced!(f32, usize, u32, i32, isize, Vec2, Vec3, Quat, Transform,);

impl<T, F: Fn(f32) -> T + Send + Sync> Sequenced<T> for F {
    fn sample_at(&self, t: f32) -> T {
        self(t)
    }
}

/// A concrete struct which holds a sequence of interpolated values of type T. Basically,
/// a keyframed animation.
pub struct Sequence<T: InterpolationPrimitive + Clone + Send + Sync> {
    /// The time at which the corresponding output should be reached.
    inputs: Vec<f32>,
    /// The sampled value at the corresponding input time. Depending on the interpolation function, 
    /// there may be multiple outputs required for a single input (for example tangents of a spline).
    outputs: Vec<T>,
    /// How to interpolate between keys
    interpolation: InterpolationFunction<T>,
    /// If the output should be normalized after being interpolated
    /// (useful when interpolating between rotations stored as Quaternions)
    normalize: bool,
}

impl<T: InterpolationPrimitive + Clone + Send + Sync> Sequence<T> {
    pub fn new(inputs: Vec<f32>, outputs: Vec<T>, interpolation: InterpolationFunction<T>, normalize: bool) -> Self {
        Sequence { inputs, outputs, interpolation, normalize }
    }

    pub fn sample(&self, t: f32) -> T {
        self.interpolation.interpolate(t, &self.inputs, &self.outputs, self.normalize)
    }
}

impl<T: InterpolationPrimitive + Clone + Send + Sync> Sequenced<T> for Sequence<T> {
    fn sample_at(&self, t: f32) -> T {
        self.sample(t)
    }
}

impl Sequenced<Vec3> for Sequence<[f32; 3]> {
    fn sample_at(&self, t: f32) -> Vec3 {
        Vec3::from(self.sample(t))
    }
}

impl Sequenced<Quat> for Sequence<[f32; 4]> {
    fn sample_at(&self, t: f32) -> Quat {
        Quat::from(Vec4::from(self.sample(t)))
    }
}

/// A convenient struct to hold the animation of a single Transform
pub struct TransformSequence<PS: Sequenced<Vec3>, OS: Sequenced<Quat>> {
    pos_seq: PS,
    ori_seq: OS,
}

impl<PS: Sequenced<Vec3>, OS: Sequenced<Quat>> TransformSequence<PS, OS> {
    pub fn new(pos_seq: PS, ori_seq: OS) -> Self {
        TransformSequence { pos_seq, ori_seq, }
    }
}

impl<PS: Sequenced<Vec3>, OS: Sequenced<Quat>> Sequenced<Transform> for TransformSequence<PS, OS> {
    fn sample_at(&self, t: f32) -> Transform {
        Transform {
            position: self.pos_seq.sample_at(t),
            orientation: self.ori_seq.sample_at(t),
        }
    }
}