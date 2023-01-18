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
(because `current <- previous - -1` is the same as
`current <- previous + 1`).

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

# Update: the compiler is smarter than me

I wrote a few different implementations of this same problem:

 - naïve C implementation, with no optimizations
 - using `fold`
 - version with explicit SIMD

Here are the benchmarks on my 2021 M1 MacBook:

```
test tests::bench_c    ... bench:         240 ns/iter (+/- 3)
test tests::bench_fold ... bench:          88 ns/iter (+/- 2)
test tests::bench_simd ... bench:         168 ns/iter (+/- 2)
```

The fold implementation is nearly twice as fast! At first, I thought
that this one wrong -- my version must be faster, right? Then I took
a peek at the assembly:

```asm
LBB1_5:
 ldp     q4, q5, [x11, #-16]
 cmgt.8h v6, v4, #0
 cmgt.8h v7, v5, #0
 sub.8h  v0, v0, v6
 sub.8h  v1, v1, v7
 usra.8h v2, v4, #15
 usra.8h v3, v5, #15
 add     x11, x11, #32
 subs    x12, x12, #16
 b.ne    LBB1_5
```

**The fold version is the same assembly, but unrolled**. It's nearly
twice as fast as my explicit SIMD version because it's nearly the same
instructions, but the loop has been unrolled so it's effectively running
two iterations of the loop in one. So, while my explicit version is
computing 8 elements in one loop iteration, this one is computing **16**.

Check out this bad boy:

    ldp     q4, q5, [x11, #-16]

That's **l**oa**d**ing a **p**air of 128-bit registers -- that's 256
bits that are being loaded in one instruction! ARM NEON specifies that
there are 32×128-bit registers, and by golly, is my compiler using them.

The rest of the instructions are unrolled versions of the loop:

    cmgt.8h v6, v4, #0
    cmgt.8h v7, v5, #0
    sub.8h  v0, v0, v6
    sub.8h  v1, v1, v7
    usra.8h v2, v4, #15
    usra.8h v3, v5, #15

The registers it's deciding to use:

 - `v0` stores the intermediate positive totals for the first 8 elements
 - `v1` stores the intermediate positive totals the latter 8 elements
 - `v2` stores the partial sums of the negatives for the first 8 elements
 - `v3` stores the partial sums of the negatives for the later 8 elements
 - `v4` first 8 elements from the array
 - `v5` latter 8 elements from the array
 - `v6` stores the comparison mask of `v4[i]>0`
 - `v7` stores the comparison mask of `v5[i]>0`

(note: I _think_ `v0`-`v7` are able to be used freely by functions, as
per the [calling conventions])

And that's why it's twice as fast as my explicit SIMD implementation.
It's literally doing twice the work in one loop iteration.


## WILD SPECULATION

> **Note**: I have no idea what I'm talking about in this section:

I am speculating that my M1 has enough execution ports to do 16 integer
ALU operations at once. Here are the instructions that I reckon can run
at the same time:

| Clock | Integer unit                               | Load/Store port                            | SIMD Port 0          | SIMD Port 1          | SIMD Port 2           | SIMD Port 3           |
|------:|--------------------------------------------|--------------------------------------------|----------------------|----------------------|-----------------------|-----------------------|
| 1     | `add x11, x11, #32`                        |                                            | `cmgt.8h v6, v4, #0` | `cmgt.8h v7, v5, #0` | `usra.8h v2, v4, #15` | `usra.8h v3, v5, #15` |
| 2     | `subs x12, x12, #16`/`b.ne LBB1_5` (fused) | `ldp q4, q5, [x11, #-16]` (next iteration) | `sub.8h  v0, v0, v6` | `sub.8h  v1, v1, v7` |                       |                       |

Yes, two clock cycles! And the math adds up:

```python
>>> clock_speed = 3_200_000_000 # 3.2 GHz for performance cores
>>> iterations = 2000 // 16
>>> cycles_per_benchmark = iterations * clocks
>>> cycles_per_benchmark / cycles_per_second * 1_000_000_000  # nanoseconds
78.125
```

(compare to 88ns/iter ±2)


Here are resources on the M1 and its execution units:

 - [Brief notes on Apple M1 Firestorm microarchitecture](https://github.com/ocxtal/insn_bench_aarch64/blob/master/optimization_notes_apple_m1.md#simdfp-domain)

Conclusion: **COMPUTERS ARE COOL**!

[problem 1]: https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
[code_report]:  https://youtu.be/U6I-Kwj-AvY
[portable_simd]: https://doc.rust-lang.org/std/simd/index.html
[Neon]: https://developer.arm.com/documentation/den0024/a/AArch64-Floating-point-and-NEON
[calling conventions]: https://developer.arm.com/documentation/102374/0101/Procedure-Call-Standard
