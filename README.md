# Bayesic

A string matching library similar to a NaiveBayes classifier, but optimized for use cases where you have many possible categories.

This is especially useful if you have two large lists of names/titles/descriptions to match with each other.

# Performance

I'm doing this project both to learn about Rust and also because I want to improve the performance of https://github.com/mmmries/bayesic by making a rust extension.
I've added a few benchmarks for training and classifying on small and large data sets (ie 60k records).

Here are the current `cargo bench` results on my laptop:

**Classification**

```
test large_classify_one_word    ... bench:       3,179 ns/iter (+/- 106)
test large_classify_three_words ... bench:       5,861 ns/iter (+/- 210)
test small_classify_one_word    ... bench:          76 ns/iter (+/- 3)
test small_classify_three_words ... bench:         197 ns/iter (+/- 4)
```

**Training**

```
test train_large ... bench:  59,907,091 ns/iter (+/- 1,243,374)
test train_small ... bench:     103,207 ns/iter (+/- 4,065)
```
