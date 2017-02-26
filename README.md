# Human_size

The `human_size` crate has sizes for humans. See the example below to see how
to use it.

```
# #![feature(try_from)]
# use human_size::{Size, Multiple};
# use std::convert::TryInto;
let my_size: Size = "1000 B".parse().expect("unable to parse size");
let same_size = Size::new(1, Multiple::Kilobyte);
assert_eq!(my_size, same_size);

println!("The size is {}", my_size); // The size is 1000 B
println!("Or {}", same_size); // Or 1 kB
let in_int: u64 = my_size.try_into().unwrap();
println!("Or even {}", in_int); // Or even 1000
```
