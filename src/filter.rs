pub trait Filter: Copy + Clone + Send {
    fn radius(&self) -> f32;
    fn evaluate(&self, p: f32) -> f32;
}

#[derive(Clone, Copy)]
pub struct BoxFilter {
    pub radius: f32,
}

impl BoxFilter {
    pub fn new(radius: f32) -> Self {
        BoxFilter { radius }
    }
}

impl Filter for BoxFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: f32) -> f32 {
        let x = p.abs();
        if x > self.radius {
            0.0
        } else {
            1.0
        }
    }
}

#[derive(Clone, Copy)]
pub struct LanczosSincFilter {
    pub radius: f32,
    pub tau: f32,
}

impl LanczosSincFilter {
    pub fn new(radius: f32, tau: f32) -> Self {
        LanczosSincFilter { radius, tau }
    }

    fn sinc(x: f32) -> f32 {
        let x = x.abs();
        if x <= 0.00001 {
            1.0
        } else {
            let pix = std::f32::consts::PI * x;
            let sin = pix.sin();
            sin / pix
        }
    }
}

impl Filter for LanczosSincFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: f32) -> f32 {
        let x = p.abs();
        if x > self.radius {
            0.0
        } else {
            let lanczos = LanczosSincFilter::sinc(x / self.tau);
            LanczosSincFilter::sinc(x) * lanczos
        }
    }
}