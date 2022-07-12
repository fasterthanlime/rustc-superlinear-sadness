# rustc-superlinear-sadness

Showcase of how tower-style code can make compile times explode on rustc
nightly as of 2022-07-12.

## Timeline

  * 2022-07-12: repro created, profiling work by @fasterthanlime, @eddyb & @BoxyUwU to scope down the problem
  * 2022-07-12: [rustc issue 99188](https://github.com/rust-lang/rust/issues/99188) opened by @eddyb

## Repo structure

All the code is in `src/main.rs`, it has no dependencies - it doesn't even
import anything outside the rust 2021 prelude.

The code will only compile if one of those `cfg` is set:

  * `assoc_type_0`, `assoc_type_1`, `assoc_type_2`, `assoc_type_3`
  * `outlives`
  * `clone`

You can "add nesting" to show how compile time evolves, by adding `--cfg more1`,
`--cfg more2`, etc. until `more7`.

## Justfile / commands

`just run --cfg assoc_type_0` will run `rustc`, using the `stage1` toolchain,
enabling self-profiling.

For the rest of the commands to work, you'll need a fork of rustc that records
`SelectionContext::evaluate_predicate_recursively` using the self-profiler.
You can use [this branch of my fork](https://github.com/fasterthanlime/rust/tree/self-profile-evaluate-predicate-recursively). If you don't, you'll still notice
compile times going up but you won't get a nice "leaderboard" like in the README
examples. 

`crox` is being run on the most recent `*.mm_profdata` file to generate a
"chrome tracing" file (as `chrome_profiler.json`), and then `jq` is used to
filter only the `evaluate_predicate_recursively` calls, which are then
sorted.

`just seq {name}` will run `just run` multiple times in a row, with a higher
`moreN` value every time, showing compile time growth. This is what is showcased
in the README.

## assoc types

With zero associated types being constrained, even with nested
`MiddleService<MiddleService<...>>` types, the number of checks is constant:

```rust
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b ()>,
```

```shell
$ just seq assoc_type_0
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
      6 Obligation(predicate=Binder(TraitPredicate(<() as std::marker::Sized>, polarity:Positive), []), depth=?)
```

<img width="484" alt="image" src="https://user-images.githubusercontent.com/7998310/178554485-44b1b591-2ab7-4d9a-914d-57e663c275bb.png">

With one associated type being constrained, it's linear:

```rust
#[cfg(assoc_type_1)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = ()>,
```

```shell
$ just seq assoc_type_1
     19 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     23 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     27 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     31 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     35 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     39 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     43 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
```

<img width="485" alt="image" src="https://user-images.githubusercontent.com/7998310/178554499-b71dcc45-6e8f-44de-859e-8e305af7075b.png">

With two, it's exponential ($2^x$ growth):

```rust
#[cfg(assoc_type_2)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = (), Error = ()>,
```

```shell
$ just seq assoc_type_2
    108 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
    220 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
    444 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
    892 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
   1788 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
   3580 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
   7164 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
```

<img width="492" alt="image" src="https://user-images.githubusercontent.com/7998310/178554515-7ff922b3-b752-49fc-afff-0f63bf7b77e3.png">

With three, it's exponential, but worse ($3^x$ growth)

```rust
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = (), Error = (), ThirdType = ()>,
```

```shell
$ just seq assoc_type_3
    403 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
   1213 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
   3643 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
  10933 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
  32803 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
  98413 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
 295243 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
```

<img width="489" alt="image" src="https://user-images.githubusercontent.com/7998310/178554540-3563073b-d5f7-4766-a792-e65015e2836d.png">

## outlives constraint

This type of constraint also shows exponential behavior ($2^x$):

```rust
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b ()>,
    for<'b> <S as Service<&'b ()>>::Future: 'b,
```

```shell
$ just seq outlives
     26 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     50 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
     98 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
    194 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
    386 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
    770 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
   1538 Obligation(predicate=Binder(TraitPredicate(<InnerService as std::marker::Sized>, polarity:Positive), []), depth=?)
```

<img width="493" alt="image" src="https://user-images.githubusercontent.com/7998310/178555618-924ca4fb-f69d-41fb-afd6-fe0f0d8f5913.png">

And a `Clone` constraint shows the exact same thing (numbers not shown here):

```rust
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b ()>,
    for<'b> <S as Service<&'b ()>>::Future: Clone,
```

## smaller repro

Here's a smaller repro of the "assoc types" explosion: [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e63793ea5031cb7aa38c64a71e49de23).

Adding another `&` in `main` makes the playground time out (be nice to the playground).

## next steps

Several issues are being filed, let's see where this goes!
