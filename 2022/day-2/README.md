# [Day 2](https://adventofcode.com/2022/day/2)

## Rock, Paper, Scissors

Some personal note on the part 1 solution:

- I don't like the `get_object_points` function. I think it's an easier way with a `Result` and an `Enum`
  but I couldn't figure it out. It works tho...
- The `HashMap` or **true_table** seems like a good solution. I don't care write all the possibilities
  (there are only 9) and with the `replace` method it makes it feel clean. However, my initial idea
  was to store it as a global with `const` but there are some caveats with how rust manages this data structures
  at compile time ([link](https://users.rust-lang.org/t/is-there-a-way-to-create-a-constant-map-in-rust/8358)).
  One solution for this is to use a crate called [phf](https://crates.io/crates/phf) but I don't want to use a
  library for this problems... I'm going to check that out in another repo maybe 👀
