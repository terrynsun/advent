Advent of Code solutions in Rust.

Each day is defined with a small framework:

```
struct Puzzle<T, R> {
    // T is the type that the input gets parsed into
    // R is the type that the answer comes in
    name: &'static str,
    parts: Vec<fn(&T) -> R>,
    delimiter: char,
    preprocess: fn(Vec<String>) -> T,
}
```

The `solve_puzzle` function:
* reads `{name}.txt`
* parses it into a `Vec<String>` by splitting on `delimiter` (usually `'\n'` but once `' '`)
* processes the Strings into the right types
* calls functions that compute both parts of the answer
