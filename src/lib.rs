#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

#[cfg(any(feature = "std"))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!(
    "Please enable feature `libm` (You used `no-default-features`, turning off `std`, but we need `libm` for `f64` math.)"
);

#[macro_use]
mod macros;

mod tweener;
mod tweens;

#[cfg(feature = "glam")]
mod glam;

pub use tweener::*;
pub use tweens::*;

/// This is the core trait of the Library, which all `tweens` implement.
///
/// Unless you choose to use a Tween directly, rather than through a [FixedTweener]
/// or [FixedTweener], you'll rarely deal with this directly.
pub trait Tween {
    /// This is the value which we tween over time.
    type Value: TweenValue;
    /// This is the kind of Time we use. For most users, it will be an `f32` or
    /// similar simple number.
    type Time: TweenTime;

    /// Run the given Tween with a new time.
    fn run(&mut self, new_time: Self::Time) -> Self::Value;

    /// The initial value a tween was set to start at.
    fn initial_value(&self) -> Self::Value;

    /// The final value the tween should end at.
    fn final_value(&self) -> Self::Value;

    /// Get a reference to the Tween's total duration.
    fn duration(&self) -> Self::Time;
}

/// This is a helper trait, which all the tweens in this library support, which gives access
/// to non-object-safe methods.
pub trait SizedTween: Tween + Sized {
    /// Creates a new `SizedTween`
    fn new(initial_value: Self::Value, final_value: Self::Value, duration: Self::Time) -> Self;
}

#[cfg(test)]
static_assertions::assert_obj_safe!(Tween<Value = i32, Time = f32>);

/// A `TweenValue` is a value which *can* be Tweened. The library fundamentally outputs
/// `TweenValue` eventually.
///
/// If you want to implement your own values to be tweened (for example, your favorite color lib),
/// then you'll need to implement this trait.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an
/// issue if that is needed for your workflow.
pub trait TweenValue: Copy {
    /// The ZERO value. Generally, this is 0 or 0.0.
    const ZERO: Self;

    /// This should be implemented as a simple subtraction. For f32, for example,
    /// it's implemented as just `destination - start`.
    fn calculate_delta(destination: Self, start: Self) -> Self;

    /// This should be implemented as a simple addition. For f32, for example,
    /// it's implemented as `self + other`.
    fn add(self, other: Self) -> Self;

    /// This should be implemented as a simple multiplication. For f32, for example,
    /// it's implemented as `(self as f64 * scale) as f32`.
    fn scale(self, scale: f64) -> Self;
}

/// A `TweenTime` is a representation of Time. The two most common will be `f32`/`f64` for
/// seconds and `u32`/`u64`/`usize` for frames.
///
/// If you want to implement your own time for duration, then you'll need to implement this
/// trait somewhere.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an
/// issue if that is needed for your workflow.
pub trait TweenTime: Copy + PartialEq {
    /// The ZERO value. Generally, this is 0 or 0.0.
    const ZERO: Self;
    /// This should be implemented as a simple division. For f32, for example,
    /// it's implemented as `(current_time / duration) as f64`.
    fn percent(duration: Self, current_time: Self) -> f64;
    /// Converts the self to an `f64`.
    fn as_f64(self) -> f64;
    /// Adds `self` to `other`. This should be implemented as simple addition.
    fn add(self, other: Self) -> Self;
    /// Subtracts `self` from `other`. This should be implemented as a simple
    /// subtraction, such as `self - other`. Notice the order.
    fn sub(self, other: Self) -> Self;
    /// This is implemented as a simple multipler, such as `self * multiplier`.
    fn scale(self, multiplier: f64) -> Self;
    /// This checks if a given time is greater than another time. For f32, for example,
    /// it's implemented as `self >= duration`.
    fn is_complete(self, duration: Self) -> bool;
}

declare_time!(u8);
declare_time!(i8);
declare_time!(i32);
declare_time!(i64);
declare_time!(u32);
declare_time!(u64);
declare_time!(usize);
declare_time!(isize);
declare_time!(float f32);
declare_time!(float f64);

declare_value!(float f32);
declare_value!(float f64);
declare_value!(u8);
declare_value!(i8);
declare_value!(i32);
declare_value!(i64);
declare_value!(u32);
declare_value!(u64);
declare_value!(usize);
declare_value!(isize);
