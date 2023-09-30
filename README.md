# Benchmarks for [Comfy](https://comfyengine.org/)

_**Disclaimer: These benchmarks are not intended to be 100% accurate
perfect representations of anything. They serve mainly as a sanity test
for comparing Comfy to other things to make sure it's fast enough that
nobody has to ever care about how fast it is.**_

This repo contains a few benchmarks for comfy, as well as the [official
bevymark](https://github.com/bevyengine/bevy/blob/v0.11.3/examples/stress_tests/bevymark.rs)
for comparison. The goal is to include more useful benchmarks over time to
make it easier to track regressions/improvements over time.

Note that bevy isn't really the target. There's other competition which is
far ahead of anything in the Rust world, for example
https://lemon07r.github.io/openfl-bunnymark/ and
https://lemon07r.github.io/kha-html5-bunnymark/ which seem to be the
fastest (around 150k on my iPhone and 300k on desktop at 60FPS).

Comfy does not aim to beat these, but we should still know how we compare.
Being slow is one thing, not know how slow and why is another :)

Comfy is happy to make tradeoffs for ergonomics and safety, but we should
do these in an informed way, and if there are free performance
improvements we can do, they should be done.

## Note on LTO

Note that the `stress-test` profile that bevy recommends includes
`lto="fat"`. I've tried the benchmark a few times with and without this
option, and at least on my machine enabling LTO leads to the same
performance (or maybe even ~1FPS worse).

Enabling this option causes the build to take 2.5 minutes longer, so considering
I can't even see the performance impact it will remain disabled.

If someone wants to contribute a better solution for running these
benchmarks or improve anything, I'm happy to accept PRs. But the point is
not 100% scientific accuracy. Right now Bevy does around 120k sprites at
around 50-55 FPS, while Comfy does around 35k sprites at ~50 FPS.

## Running benchmarks

Comfy bunnymark

```sh
make comfymark
```

Comfy bunnymark with [Tracy](https://github.com/wolfpld/tracy) enabled

```sh
make comfymark-tracy
```

Bevy bunnymark

```sh
make bevymark
```

Asset loading benchmark (WIP)

```sh
make asset-benchmark
```
