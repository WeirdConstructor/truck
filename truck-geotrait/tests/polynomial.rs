
use truck_base::cgmath64::*;
use truck_geotrait::*;

// polynomial curve
#[derive(Clone, Debug)]
pub struct PolyCurve<P: EuclideanSpace<Scalar = f64>>(pub Vec<P::Diff>);

impl<P: EuclideanSpace<Scalar = f64>> ParametricCurve for PolyCurve<P> {
    type Point = P;
    type Vector = P::Diff;
    #[inline(always)]
    fn subs(&self, t: f64) -> P {
        self.0
            .iter()
            .fold((1.0, P::origin()), |(s, res), a| (s * t, res + *a * s))
            .1
    }
    fn der(&self, t: f64) -> P::Diff {
        self.0
            .iter()
            .enumerate()
            .skip(1)
            .fold((1.0, P::Diff::zero()), |(s, res), (deg, a)| {
                (s * t, res + *a * s * deg as f64)
            })
            .1
    }
    fn der2(&self, t: f64) -> P::Diff {
        self.0
            .iter()
            .enumerate()
            .skip(2)
            .fold((1.0, P::Diff::zero()), |(s, res), (deg, a)| {
                (s * t, res + *a * s * (deg * (deg - 1)) as f64)
            })
            .1
    }
    fn parameter_range(&self) -> (f64, f64) { (-100.0, 100.0) }
}

// surface by tensor product of polynomials e.g. `(2u^2 + 3u + 1)(4v^2 - 6v + 2)`
#[derive(Clone, Debug)]
pub struct PolySurface(pub PolyCurve<Point3>, pub PolyCurve<Point3>);

impl ParametricSurface for PolySurface {
    type Point = Point3;
    type Vector = Vector3;
    #[inline(always)]
    fn subs(&self, u: f64, v: f64) -> Point3 { self.0.subs(u).mul_element_wise(self.1.subs(v)) }
    #[inline(always)]
    fn uder(&self, u: f64, v: f64) -> Vector3 {
        self.0.der(u).mul_element_wise(self.1.subs(v).to_vec())
    }
    #[inline(always)]
    fn vder(&self, u: f64, v: f64) -> Vector3 {
        self.0.subs(u).to_vec().mul_element_wise(self.1.der(v))
    }
    #[inline(always)]
    fn uuder(&self, u: f64, v: f64) -> Vector3 {
        self.0.der2(u).mul_element_wise(self.1.subs(v).to_vec())
    }
    #[inline(always)]
    fn uvder(&self, u: f64, v: f64) -> Vector3 { self.0.der(u).mul_element_wise(self.1.der(v)) }
    #[inline(always)]
    fn vvder(&self, u: f64, v: f64) -> Vector3 {
        self.0.subs(u).to_vec().mul_element_wise(self.1.der2(v))
    }
    #[inline(always)]
    fn normal(&self, u: f64, v: f64) -> Vector3 {
        self.uder(u, v).cross(self.vder(u, v)).normalize()
    }
}
