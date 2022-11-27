# Longest common prefix

In this problem, you'll code a function to compute longest common prefix of multiple strings. You're allowed to make additional allocations.

## UTF-8

Remember: Rust strings are UTF-8! That means we'll consider them as input too. Moreover, it's not clear when two UTF-8 strings are equal.

Consider the word **café**:

This can be represented as:

`A = [U+0063 U+0061 U+0066 U+0065 U+0301]` (ends with **e** and a **combining accent**)

But also as

`B = [U+0063 U+0061 U+0066 U+00E9]` (ends with **é**, the combined form)

To be clear, we are searching for the longest prefix of equal [`.chars()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.chars).

## Useful links

- [`.as_bytes()`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes) method.
- [`.char_indices()`](https://doc.rust-lang.org/std/string/struct.String.html#method.char_indices) method.

## Complexity

Your solution must be O(n), where N is the total length of the strings. Beware of [`.nth()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth): it's linear!

## Advanced level

- Write a solution that doesn't allocate anything.
