# tween

![docs.rs](https://img.shields.io/docsrs/tween)
![Crates.io](https://img.shields.io/crates/v/tween)
![Crates.io](https://img.shields.io/crates/l/tween)

`tween` is an std-optional tweening library, designed for use in games and animations.

To install, add the following to your Cargo.toml:

```toml
tween = "2.0.0"
```

<!-- This trait exports a trait `Tween`, a variety of structs which implement common tweens (such as `Elastic`, `Quart`, etc), and two `Tweeners`, which wrap around tweens, allowing users to drive them trivially.

First, tweens are simple to create:

```rust
use tween::Tweener;

let mut sine_in = Tweener::sine_in(0.0, 200.0, 60);
let _value0 = sine_in.run(0);
let _value1 = sine_in.run(1);
let _value2 = sine_in.run(2);
let _value3 = sine_in.run(3);
```

However, as the above example shows, it's more typical that you'll want to simply drive a tween over time, rather than giving arbitrary times within a duration.

For that, we use tweeners, which come in two kinds: `Tweener` and `FixedTweenDriver`. For both, users provide *deltas*, rather than arbitrary times. A `FixedTweenDriver` uses just *one* delta (appropriate in a game engine with a Fixed Update pipeline), whereas a `Tweener` can take a fixed delta.

```rust
use tween::{SineIn, FixedTweenDriver};

let (start, end) = (0.0, 200.0);
let duration = 60;

let sine_in = SineIn::new(start, end, duration);
let delta = 1;
let mut sine_in_tweener = FixedTweenDriver::new(sine_in, delta);

let _value0: Option<f32> = sine_in_tweener.next();
let _value1: Option<f32> = sine_in_tweener.next();
let _value2: Option<f32> = sine_in_tweener.next();
let _value3: Option<f32> = sine_in_tweener.next();

// we can finish off the tweener by using the iterator interface.
// we only use `&mut` here to demonstrate the `None` after this loop, you can just
// take it by value like normal.
for value in &mut sine_in_tweener {
    // FixedTweenDriver provides an iterator interface
}

assert!(sine_in_tweener.next().is_none());
```

## Std Optional

This library uses `std` with the default feature `std`. Disable default features, and enable `libm`, for a no-std experience. (We need to use `libm` for the floating point math).

## Generics

This library uses generics heavily. There are two core generics used: `TweenValue` and `TweenTime`. All built-in numeric types implement both traits. For your own code, you can implement either trait. For example, you could easily implement `TweenValue` for your favorite math library. -->
