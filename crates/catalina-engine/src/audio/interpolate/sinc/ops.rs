#![allow(dead_code)]

pub mod f64 {
    #[cfg(not(feature = "std"))]
    pub fn sin(x: f64) -> f64 {
        libm::sin(x)
    }

    #[cfg(feature = "std")]
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    #[cfg(not(feature = "std"))]
    pub fn cos(x: f64) -> f64 {
        libm::cos(f)
    }

    #[cfg(feature = "std")]
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }
}
