//! Contains the implementation of the Ristretto protocol.


use crate::constants;
use crate::scalar::Ristretto255Scalar;
use crate::edwards::EdwardsPoint;
use crate::field::FieldElement;
use crate::traits::ops::*;
use crate::traits::Identity;

use core::ops::{Add, Sub, Mul, Neg, Index};

use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq};


/// Ristretto Point expressed in wire format.
/// Since the Ristretto bytes encoding is canonical,
/// two points are equal if their encodin form is equal. 
pub struct CompressedRistretto(pub [u8; 32]);

impl Index<usize> for CompressedRistretto {
    type Output = u8;
    fn index(&self, _index: usize) -> &u8 {
        &(self.0[_index])
    }
}

impl ConstantTimeEq for CompressedRistretto {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_bytes().ct_eq(&other.as_bytes())
    }
}

impl CompressedRistretto {
    fn as_bytes(&self) -> [u8; 32] {
        self.0
    }
}


pub struct RistrettoPoint (pub(crate) EdwardsPoint);

