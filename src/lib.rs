#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler))]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate core;

pub mod arith;
mod fields;
mod groups;
pub mod prelude;

use fields::FieldElement;
use groups::GroupElement;

pub use crate::{fields::FieldError, groups::AffineGError};
use core::{
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

    pub fn from_u256(u256: arith::U256) -> Result<Self, FieldError> {
        Ok(Fr(fields::Fr::new(u256)?))
    }

    pub fn into_u256(self) -> arith::U256 {
        self.0.into()
    }

    pub fn to_big_endian(self) -> [u8; 32] {
        let u256: arith::U256 = self.0.into();
        u256.to_big_endian()
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, FieldError> {
        Ok(Fr(fields::Fr::new(arith::U256::from_slice(slice))?))
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
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fr(fields::Fr::from_str(s)?))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct Fq(fields::Fq);

impl Fq {
    pub fn zero() -> Self {
        Fq(fields::Fq::zero())
    }

    pub fn one() -> Self {
        Fq(fields::Fq::one())
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Fq(fields::Fq::random(rng))
    }

    pub fn pow(&self, exp: Fq) -> Self {
        Fq(self.0.pow(exp.0))
    }

    pub fn inverse(&self) -> Option<Self> {
        self.0.inverse().map(Fq)
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn interpret(buf: &[u8; 64]) -> Fq {
        Fq(fields::Fq::interpret(buf))
    }

    pub fn from_u256(u256: arith::U256) -> Result<Self, FieldError> {
        Ok(Fq(fields::Fq::new(u256)?))
    }

    pub fn into_u256(self) -> arith::U256 {
        self.0.into()
    }

    pub fn to_big_endian(self) -> [u8; 32] {
        let u256: arith::U256 = self.0.into();
        u256.to_big_endian()
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, FieldError> {
        Ok(Fq(fields::Fq::new(arith::U256::from_slice(slice))?))
    }
}

impl Add<Fq> for Fq {
    type Output = Fq;

    fn add(self, other: Fq) -> Fq {
        Fq(self.0 + other.0)
    }
}

impl Sub<Fq> for Fq {
    type Output = Fq;

    fn sub(self, other: Fq) -> Fq {
        Fq(self.0 - other.0)
    }
}

impl Neg for Fq {
    type Output = Fq;

    fn neg(self) -> Fq {
        Fq(-self.0)
    }
}

impl Mul for Fq {
    type Output = Fq;

    fn mul(self, other: Fq) -> Fq {
        Fq(self.0 * other.0)
    }
}

impl FromStr for Fq {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fq(fields::Fq::from_str(s)?))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Fq2(fields::Fq2);

impl Fq2 {
    pub fn new(a: Fq, b: Fq) -> Fq2 {
        Fq2(fields::Fq2::new(a.0, b.0))
    }

    pub fn one() -> Fq2 {
        Fq2(fields::Fq2::one())
    }

    pub fn zero() -> Fq2 {
        Fq2(fields::Fq2::zero())
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn pow(&self, exp: arith::U256) -> Self {
        Fq2(self.0.pow(exp))
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, FieldError> {
        let u512 = arith::U512::from_slice(bytes);
        let (res, c0) = u512.divrem(&fields::Fq::modulus());
        Ok(Fq2::new(
            Fq::from_u256(c0).map_err(|_| FieldError::InvalidMember)?,
            Fq::from_u256(res.ok_or(FieldError::InvalidMember)?).map_err(|_| FieldError::InvalidMember)?
        ))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct AffineG1(groups::AffineG1);

impl AffineG1 {
    pub fn new(x: Fq, y: Fq) -> Result<Self, AffineGError> {
        Ok(AffineG1(groups::AffineG1::new(x.0, y.0)?))
    }

    pub fn x(&self) -> Fq {
        Fq(*self.0.x())
    }

    pub fn y(&self) -> Fq {
        Fq(*self.0.y())
    }

    pub fn from_jacobian(g1: G1) -> Option<Self> {
        g1.0.to_affine().map(AffineG1)
    }
}

impl From<AffineG1> for G1 {
    fn from(affine: AffineG1) -> Self {
        G1(affine.0.to_jacobian())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct AffineG2(groups::AffineG2);

impl AffineG2 {
    pub fn new(x: Fq2, y: Fq2) -> Result<Self, AffineGError> {
        Ok(AffineG2(groups::AffineG2::new(x.0, y.0)?))
    }

    pub fn from_jacobian(g2: G2) -> Option<Self> {
        g2.0.to_affine().map(AffineG2)
    }
}

impl From<AffineG2> for G2 {
    fn from(affine: AffineG2) -> Self {
        G2(affine.0.to_jacobian())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{String, Vec};

    pub fn into_hex<S: serde::Serialize>(obj: S) -> Option<String> {
        bincode::serialize(&obj).ok().map(hex::encode)
    }

    pub fn from_hex<D: serde::de::DeserializeOwned>(s: &str) -> Option<D> {
        let s = hex::decode(s).unwrap();
        bincode::deserialize(&s).ok()
    }

    pub fn reserialize(obj: Vec<u8>) -> Vec<u8> {
        let s = into_hex(obj).unwrap();

        from_hex(&s).unwrap()
    }

    #[test]
    fn group_serialization_and_deserialization() {
        let b = Fr::random(&mut rand::thread_rng());

        let mut a = G1::one();
        for _ in 0..100 {
            a = a * b;

            let a_ser = bincode::serialize(&a).unwrap();
            assert_eq!(reserialize(a_ser.clone()), a_ser.clone());
            assert_eq!(reserialize(reserialize(a_ser.clone())), a_ser);
            let mut c = a;
            c.normalize();

            assert_eq!(a, c);
        }

        let mut a = G2::one();
        for _ in 0..100 {
            a = a * b;

            let a_ser = bincode::serialize(&a).unwrap();
            assert_eq!(reserialize(a_ser.clone()), a_ser);
            assert_eq!(reserialize(reserialize(a_ser.clone())), a_ser);
            let mut c = a;
            c.normalize();

            assert_eq!(a, c);
        }
    }

    // TODO: Need to get the right new serialised hex and retest these last bits.
    #[test]
    fn group_serialization_edge_cases() {
        assert_eq!(from_hex::<G1>("00").unwrap(), G1::zero());
        assert_eq!(from_hex::<G2>("00").unwrap(), G2::zero());
        assert!(from_hex::<G1>("23").is_none());
        assert!(from_hex::<G2>("23").is_none());

        // not points on the curve
        // assert!(from_hex::<G1>("04177cedb64589bde7a64ad24f89bbb8c9f05535810865aaea8fbf8184ff9e120313500226b2422d2068614d1c8c7146c806a97743e78d9901748a9ded08ea9e5f").is_none());
        // assert!(from_hex::<G2>("0404d4bf3239f77cee7b47c7245e9280b3e9c1182d6381a87bbf81f9f2a6254b731df569cda95e060bee91ba69b3f2d103658a7aea6b10e5bdc761e5715e7ee4bb01b4c328f0cbdb4aada63b3d09100d792376b94d07a6004e46054eeec849e8de9835158a11d28483dd8db236ea49f3630edc9e41944e494c5aacfc36af3b66e7").is_none());

        // out of bounds
        // assert!(from_hex::<G2>("04ffd6a64a62b843a22c6250eda2354d603e74c30ed0b1435951c3f7dd541538beb8a43915823125c6bb89aece89125664ce78ca69b81cdb8164b40eb2833560b606e11258ce33c4076eff0d5824f210466b588d324b60ccd5a2b7f180f9a7cd7f1ab564ddb03b1b684ff4315acc6eef5229d99fe107afaea83a5c72f2b4c33aca").is_none());
    }
}
