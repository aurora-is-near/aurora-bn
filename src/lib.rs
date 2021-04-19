pub mod arith;
mod fields;
mod groups;

use fields::FieldElement;
use groups::GroupElement;

use rand::Rng;
use std::ops::{Add, Mul, Neg, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct Fr(fields::Fr);

impl Fr {
    pub fn zero() -> Self {
        Fr(fields::Fr::zero())
    }

    pub fn one() -> Self {
        Fr(fields::Fr::one())
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Fr(fields::Fr::random(rng))
    }

    pub fn pow(&self, exp: Fr) -> Self {
        Fr(self.0.pow(exp.0))
    }

    pub fn inverse(&self) -> Option<Self> {
        self.0.inverse().map(Fr)
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn interpret(buf: &[u8; 64]) -> Fr {
        Fr(fields::Fr::interpret(buf))
    }
}

impl Add<Fr> for Fr {
    type Output = Fr;

    fn add(self, other: Fr) -> Fr {
        Fr(self.0 + other.0)
    }
}

impl Sub<Fr> for Fr {
    type Output = Fr;

    fn sub(self, other: Fr) -> Fr {
        Fr(self.0 - other.0)
    }
}

impl Neg for Fr {
    type Output = Fr;

    fn neg(self) -> Fr {
        Fr(-self.0)
    }
}

impl Mul for Fr {
    type Output = Fr;

    fn mul(self, other: Fr) -> Fr {
        Fr(self.0 * other.0)
    }
}

impl FromStr for Fr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fr(fields::Fr::from_str(s)?))
    }
}

pub trait Group:
    serde::Serialize
    + serde::Deserialize<'static>
    + 'static
    + Send
    + Sync
    + Copy
    + Clone
    + PartialEq
    + Eq
    + Sized
    + Add
    + Sub
    + Neg
    + Mul<Fr>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn random<R: Rng>(rng: &mut R) -> Self;
    fn is_zero(&self) -> bool;
    fn normalize(&mut self);
}

#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct G1(groups::G1);

impl Group for G1 {
    fn zero() -> Self {
        G1(groups::G1::zero())
    }

    fn one() -> Self {
        G1(groups::G1::one())
    }

    fn random<R: Rng>(rng: &mut R) -> Self {
        G1(groups::G1::random(rng))
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn normalize(&mut self) {
        let new = match self.0.to_affine() {
            Some(a) => a,
            None => return,
        };

        self.0 = new.to_jacobian();
    }
}

impl Add for G1 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        G1(self.0 + other.0)
    }
}

impl Sub for G1 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        G1(self.0 - other.0)
    }
}

impl Neg for G1 {
    type Output = Self;

    fn neg(self) -> Self {
        G1(-self.0)
    }
}

impl Mul<Fr> for G1 {
    type Output = Self;

    fn mul(self, other: Fr) -> Self {
        G1(self.0 * other.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct G2(groups::G2);

impl Group for G2 {
    fn zero() -> Self {
        G2(groups::G2::zero())
    }

    fn one() -> Self {
        G2(groups::G2::one())
    }

    fn random<R: Rng>(rng: &mut R) -> Self {
        G2(groups::G2::random(rng))
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn normalize(&mut self) {
        let new = match self.0.to_affine() {
            Some(a) => a,
            None => return,
        };

        self.0 = new.to_jacobian();
    }
}

impl Add for G2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        G2(self.0 + other.0)
    }
}

impl Sub<G2> for G2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        G2(self.0 - other.0)
    }
}

impl Neg for G2 {
    type Output = Self;

    fn neg(self) -> Self {
        G2(-self.0)
    }
}

impl Mul<Fr> for G2 {
    type Output = Self;

    fn mul(self, other: Fr) -> Self {
        G2(self.0 * other.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Gt(fields::Fq12);

impl Gt {
    pub fn one() -> Self {
        Gt(fields::Fq12::one())
    }
    pub fn pow(&self, exp: Fr) -> Self {
        Gt(self.0.pow(exp.0))
    }
    pub fn inverse(&self) -> Self {
        Gt(self.0.inverse().unwrap())
    }
}

impl Mul<Gt> for Gt {
    type Output = Gt;

    fn mul(self, other: Gt) -> Gt {
        Gt(self.0 * other.0)
    }
}

pub fn pairing(p: G1, q: G2) -> Gt {
    Gt(groups::pairing(&p.0, &q.0))
}
