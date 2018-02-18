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
