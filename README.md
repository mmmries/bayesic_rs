# Bayesic

A string matching library similar to a NaiveBayes classifier, but optimized for use cases where you have many possible matches.

This is especially useful if you have two large lists of names/titles/descriptions to match with each other.

# Performance

I'm doing this project both to learn about Rust and also because I want to improve the performance of https://github.com/mmmries/bayesic by making a rust extension.
I've added a few benchmarks for training and classifying on small and large data sets (ie 60k records).

Here are the current un-optimized results:

**Classification**

```
test large_classify_one_word    ... bench:       4,831 ns/iter (+/- 898)
test large_classify_three_words ... bench:   1,911,829 ns/iter (+/- 224,408)
test small_classify_one_word    ... bench:         950 ns/iter (+/- 183)
test small_classify_three_words ... bench:       2,304 ns/iter (+/- 216)
```

**Training**

```
test train_large ... bench:  99,597,005 ns/iter (+/- 26,388,979)
test train_small ... bench:     127,263 ns/iter (+/- 14,402)
```