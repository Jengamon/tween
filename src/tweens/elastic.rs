use crate::{Tween, Tween2, TweenTime, TweenValue};
use core::{f64::consts::PI, marker::PhantomData};

/// An elastic tween in. Go [here](https://easings.net/#easeInElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticIn<TValue, TTime> {
    value_delta: TValue,
    initial_value: TValue,
    final_value: TValue,
    duration: TTime,
    three_tenths: f64,
    s: f64,
}

impl<TValue, TTime> ElasticIn<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
        <Self as crate::SizedTween>::new(initial_value, final_value, duration)
    }
}

impl<V, T> Tween for ElasticIn<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        if t == 0.0 {
            return self.initial_value;
        }

        if t == 1.0 {
            return self.final_value;
        }

        let t: f64 = t - 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::pow(2.0, t * 10.0);

        #[cfg(feature = "std")]
        let scalar = 2f64.powf(t * 10.0);

        let post_fix = self.value_delta.scale(scalar);
        let temp = (self.duration.to_f64() * t - self.s) * (2.0 * PI) / self.three_tenths;

        #[cfg(feature = "libm")]
        let scalar = -libm::sin(temp);

        #[cfg(feature = "std")]
        let scalar = -temp.sin();

        post_fix.scale(scalar) + self.initial_value
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn initial_value(&self) -> V {
        self.initial_value
    }

    fn final_value(&self) -> V {
        self.final_value
    }
}

impl<V, T> crate::SizedTween for ElasticIn<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn new(initial_value: V, final_value: V, duration: T) -> Self {
        let delta = final_value - initial_value;
        let three_tenths = duration.to_f64() * 0.3;
        Self {
            value_delta: delta,
            duration,
            three_tenths,
            s: three_tenths * 0.25,
            initial_value,
            final_value,
        }
    }
}

/// An elastic tween out. Go [here](https://easings.net/#easeOutElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticOut<TValue, TTime> {
    initial_value: TValue,
    final_value: TValue,
    value_delta: TValue,
    duration: TTime,
    three_tenths: f64,
    s: f64,
}

impl<TValue, TTime> ElasticOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
        <Self as crate::SizedTween>::new(initial_value, final_value, duration)
    }
}

impl<V, T> Tween for ElasticOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        if t == 0.0 {
            return self.initial_value;
        }

        if t == 1.0 {
            return self.final_value;
        }

        let temp = (t * self.duration.to_f64() - self.s) * (2.0 * PI) / self.three_tenths;

        #[cfg(feature = "libm")]
        let scalar = libm::pow(2.0, -10.0 * t) * libm::sin(temp);

        #[cfg(feature = "std")]
        let scalar = 2f64.powf(-10.0 * t) * temp.sin();

        self.value_delta.scale(scalar) + self.value_delta + self.initial_value
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn initial_value(&self) -> V {
        self.initial_value
    }

    fn final_value(&self) -> V {
        self.final_value
    }
}

impl<V, T> crate::SizedTween for ElasticOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn new(initial_value: V, final_value: V, duration: T) -> Self {
        let delta = final_value - initial_value;
        let three_tenths = duration.to_f64() * 0.3;
        Self {
            value_delta: delta,
            duration,
            three_tenths,
            s: three_tenths * 0.25,
            initial_value,
            final_value,
        }
    }
}

/// An elastic tween in and out. Go [here](https://easings.net/#easeInOutElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticInOut<TValue, TTime> {
    initial_value: TValue,
    final_value: TValue,
    value_delta: TValue,
    duration: TTime,
    p: f64,
    s: f64,
}

impl<TValue, TTime> ElasticInOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
        <Self as crate::SizedTween>::new(initial_value, final_value, duration)
    }
}

impl<V, T> Tween for ElasticInOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        if t == 0.0 {
            return self.initial_value;
        }

        if t == 2.0 {
            return self.final_value;
        }

        let t = t - 1.0;
        if t < 0.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, t * 10.0);

            #[cfg(feature = "std")]
            let scalar = 2f64.powf(t * 10.0);

            let post_fix = self.value_delta.scale(scalar);
            let temp = (self.duration.to_f64() * t - self.s) * (2.0 * PI) / self.p;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sin(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(-0.5 * temp_sin) + self.initial_value
        } else {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, t * -10.0);

            #[cfg(feature = "std")]
            let scalar = 2f64.powf(-10.0 * t);

            let post_fix = self.value_delta.scale(scalar);
            let temp = (self.duration.to_f64() * t - self.s) * (2.0 * PI) / self.p;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sin(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(temp_sin * 0.5) + self.final_value
        }
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn initial_value(&self) -> V {
        self.initial_value
    }

    fn final_value(&self) -> V {
        self.final_value
    }
}

impl<V, T> crate::SizedTween for ElasticInOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn new(initial_value: V, final_value: V, duration: T) -> Self {
        let delta = final_value - initial_value;
        let p = duration.to_f64() * 0.45;
        Self {
            value_delta: delta,
            duration,
            p,
            s: p * 0.25,
            initial_value,
            final_value,
        }
    }
}

pub struct ElasticIn2<Value, Time> {
    duration: Time,
    three_tenths: f64,
    s: f64,
    _value: Value,
}
impl<Value, Time> Tween2<Value> for ElasticIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        if percent == 0.0 {
            return Value::ZERO;
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent -= 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::pow(2.0, percent * 10.0);

        #[cfg(feature = "std")]
        let scalar = 2f64.powf(percent * 10.0);

        let post_fix = value_delta.scale(scalar);
        let temp = (self.duration.to_f64() * percent - self.s) * (2.0 * PI) / self.three_tenths;

        #[cfg(feature = "libm")]
        let scalar = -libm::sin(temp);

        #[cfg(feature = "std")]
        let scalar = -temp.sin();

        post_fix.scale(scalar)
    }
}

pub struct ElasticOut2<Value, Time> {
    duration: Time,
    three_tenths: f64,
    s: f64,
    _value: Value,
}
impl<Value, Time> Tween2<Value> for ElasticOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        if percent == 0.0 {
            return Value::ZERO;
        }

        if percent == 1.0 {
            return value_delta;
        }

        let temp = (percent * self.duration.to_f64() - self.s) * (2.0 * PI) / self.three_tenths;

        #[cfg(feature = "libm")]
        let scalar = libm::pow(2.0, -10.0 * percent) * libm::sin(temp);

        #[cfg(feature = "std")]
        let scalar = 2f64.powf(-10.0 * percent) * temp.sin();

        value_delta.scale(scalar) + value_delta
    }
}

pub struct ElasticInOut2<Value, Time> {
    duration: Time,
    p: f64,
    s: f64,
    _value: Value,
}
impl<Value, Time> Tween2<Value> for ElasticInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        if percent == 0.0 {
            return Value::ZERO;
        }

        if percent == 2.0 {
            return value_delta;
        }

        percent -= 1.0;

        if percent < 0.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, percent * 10.0);

            #[cfg(feature = "std")]
            let scalar = 2f64.powf(percent * 10.0);

            let post_fix = value_delta.scale(scalar);
            let temp = (self.duration.to_f64() * percent - self.s) * (2.0 * PI) / self.p;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sin(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(-0.5 * temp_sin)
        } else {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, percent * -10.0);

            #[cfg(feature = "std")]
            let scalar = 2f64.powf(-10.0 * percent);

            let post_fix = value_delta.scale(scalar);
            let temp = (self.duration.to_f64() * percent - self.s) * (2.0 * PI) / self.p;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sin(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(temp_sin * 0.5) + value_delta
        }
    }
}

test_tween!(Elastic);
