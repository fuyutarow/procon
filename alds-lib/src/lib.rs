pub mod collections;

pub mod monoid;
pub use monoid::Monoid;

/// impl_monoid!(Max, usize, 0, |&x,&y| std::cmp::max(x,y));
#[macro_export]
macro_rules! impl_monoid {
    ($m:ident, $t:ty, $id:expr, $op:expr) => {
        pub struct $m;
        impl Monoid for $m {
            type Value = $t;
            fn id() -> Self::Value {
                $id
            }
            fn op(x: &Self::Value, y: &Self::Value) -> Self::Value {
                $op(x, y)
            }
        }
    };
}

#[macro_export]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

#[macro_export]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[macro_export]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

#[macro_export]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}
