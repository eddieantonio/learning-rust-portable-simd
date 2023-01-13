# Teaching myself SIMD

Solving [“Maximum Count of Positive Integer and Negative
Integer”][problem 1], a.k.a., “problem 1” from [this code_report
video][code_report].

I used Rust's [`portable_simd`][portable_simd] feature, which, as of 2023-01-13, is only
available in nightly.

Despite using the "portable" SIMD library, I decided to focus on my own
development machine, a 2021 M1 Macbook. The M1 chip implements the
[AArch64 NEON][Neon] feature, which features 128-bit SIMD registers.
128-bits = 8 lanes * 16-bit signed integers.

The kernel is tiny -- only 6 instructions long:

```asm
LBB0_11:
 ldr     q2, [x12], #16
 cmgt.8h v3, v2, #0
 sub.8h  v1, v1, v3
 usra.8h v0, v2, #15
 subs    x13, x13, #16
 b.ne    LBB0_11
```

# Explanation

    ldr     q2, [x12], #16

Loads 8 16-bit signed integers into 128-bit register `q2` while also
incrementing the data pointer `x12` by 16 bytes (8 × 2 bytes per
element). From now on, `v2` means treat register `q2` as a vector of
multiple values, rather than a single value.

    cmgt.8h v3, v2, #0

Compares 8 16-bit signed integers against 0. The result is stored in
`v3`, where `true` is represented by -1 (all bits set to 1 in
twos-complement representation).

    sub.8h  v1, v1, v3

Subtracts the result of the previous comparison from the accumulator
vector `v1`. Because `true` is represented by -1, we need to subtract
(because `current <- previous - -1` is the same as `current <- previous
+ 1`).

    usra.8h v0, v2, #15

This one is funky! `usra` stands for "unsigned shift right and
accumulate". We shift each element from `v2` right by 15, pretending
they're unsigned values. In twos-complement representation, this
effectively leaves us with a sign bit: we get 1 if the result is less
than zero or 0 otherwise. We accumulate that result in `v0`.

    subs    x13, x13, #16
    b.ne    LBB0_11

`x13` is initialized to the amount of bytes to read from the vector.
This number is _always_ aligned to 16-bytes. So these two instructions
effectively mean "we have seen 16 bytes of data -- loop if there bytes
left to process."

---

The entire problem with 2000 elements runs faster than I can measure it
(under 1ms), so it's fast!

[problem 1]: https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
[code_report]:  https://youtu.be/U6I-Kwj-AvY
[portable_simd]: https://doc.rust-lang.org/std/simd/index.html
[Neon]: https://developer.arm.com/documentation/den0024/a/AArch64-Floating-point-and-NEON
