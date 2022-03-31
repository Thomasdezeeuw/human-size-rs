# v0.4.2

* Added `SpecificSize::to_bytes`.

# v0.4.1

* Added `enable-serde` feature enable implementations of `Serialize` and
  `Deserialize` for `SpecificSize`.

# v0.4.0

* **BREAKING**: Rewrote the entire module.
* Added `SpecificSize`.
* Added `Multiple` trait.
* Added `multiples` module, contains implementations of the `Multiple` trait.
* Added type alias `Size`, somewhat similar to the old `Size` of v0.3.

# v0.3.0

* **BREAKING**: Removed `UnknownExtra` variant from `ParsingError`, no longer used.
* Allow no whitespace between the value and the multiple, e.g. `1kb` is now allowed.
* Added `EmptyInput` variant to `ParsingError`.

# v0.2.0

* Changed value from `u32` to `f64`.
* Remove `TryInto` implementations for `Size` and `Multiple`.
* Added `Size.into_bytes`.
* Remove large multiples; `Exabyte`, `Zettabyte`, `Yottabyte` and `Exbibyte`,
  `Zebibyte`, `Yobibyte`, see issue #2.

# v0.1.0

Initial release.
