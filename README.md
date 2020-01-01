# Bayesic

A string matching library similar to a NaiveBayes classifier, but optimized for use cases where you have many possible matches.

This is especially useful if you have two large lists of names/titles/descriptions to match with each other.

# Performance

I'm doing this project both to learn about Rust and also because I want to improve the performance of https://github.com/mmmries/bayesic by making a rust extension.
I've added a few benchmarks for training and classifying on small and large data sets (ie 60k records).

Here are the current un-optimized results:

**Classification**

```
test large_classify_one_word    ... bench:      10,442 ns/iter (+/- 740)
test large_classify_three_words ... bench:   2,781,003 ns/iter (+/- 129,022)
test small_classify_one_word    ... bench:       1,557 ns/iter (+/- 221)
test small_classify_three_words ... bench:       3,985 ns/iter (+/- 436)
```

**Training**

```
test train_large ... bench: 111,759,760 ns/iter (+/- 7,154,122)
test train_small ... bench:     218,304 ns/iter (+/- 58,278)
```