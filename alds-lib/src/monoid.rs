/// https://github.com/tanakh/competitive-rs/blob/master/src/monoid.rs
use num::{Bounded, One, Zero};
use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

/// A trait of monoids
///
/// The class of monoids (types with an associative binary operation that has an identity). Instances should satisfy the following laws:
/// * `mappend(x, MEMPTY) = x`
/// * `mappend(MEMPTY, x) = x`
/// * `mappend(x, mappend(y, z)) = mappend(mappend(x, y), z)` (Semigroup law)
///
/// * `op(op(x, y), z) == op(x, op(x, y))`
/// * `op(id(), x) == x == op(id(), x)`
///
///
/// ```
/// use monoid::Monoid;
/// enum Op {}
/// impl Monoid for Op{
///     type Value = i32;
///     fn id() -> i32 {
///         0
///     }
///     fn op(x: &i32, y: &i32) -> i32 {
///         x + y
///     }
///
/// }
/// ```
pub trait Monoid: Sized {
    type Value: Debug + Default;

    fn id() -> Self::Value;

    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;

    /// Identity of `mappend`
    fn mempty() -> Self::Value {
        Self::id()
    }

    /// An associative operation
    fn mappend(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        Self::op(lhs, rhs)
    }

    /// Fold a slice using the monoid
    fn mconcat(xs: &[Self::Value]) -> Self::Value {
        xs.iter().fold(Self::mempty(), |a, b| Self::mappend(&a, b))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sum<T>(pub T);

impl<T> Monoid for Sum<T>
where
    T: Copy + Zero + Add<Output = T> + Debug + Default,
{
    type Value = T;

    fn id() -> Self::Value {
        T::zero()
    }

    fn op(&x: &T, &y: &T) -> Self::Value {
        x + y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Product<T>(pub T);

impl<T> Monoid for Product<T>
where
    T: Copy + One + Mul<Output = T> + Debug + Default,
{
    type Value = T;

    fn id() -> Self::Value {
        T::one()
    }

    fn op(&x: &T, &y: &T) -> Self::Value {
        x * y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Max<T>(pub T);

impl<T> Monoid for Max<T>
where
    T: Copy + Ord + Bounded + Debug + Default,
{
    type Value = T;

    fn id() -> Self::Value {
        <T as Bounded>::min_value()
    }

    fn op(&x: &T, &y: &T) -> Self::Value {
        x.max(y)
    }
}

impl<T> From<T> for Max<T> {
    fn from(v: T) -> Self {
        Max(v)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Min<T>(pub T);

impl<T> Monoid for Min<T>
where
    T: Copy + Ord + Bounded + Debug + Default,
{
    type Value = T;

    fn id() -> Self::Value {
        <T as Bounded>::max_value()
    }

    fn op(&x: &T, &y: &T) -> Self::Value {
        x.min(y)
    }
}

impl<T> From<T> for Min<T> {
    fn from(v: T) -> Self {
        Min(v)
    }
}
