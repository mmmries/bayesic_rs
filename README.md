# Bayesic

A string matching library similar to a NaiveBayes classifier, but optimized for use cases where you have many possible categories.

This is especially useful if you have two large lists of names/titles/descriptions to match with each other.

# Performance

I'm doing this project both to learn about Rust and also because I want to improve the performance of https://github.com/mmmries/bayesic by making a rust extension.
I've added a few benchmarks for training and classifying on small and large data sets (ie 60k records).

Here are the current un-optimized results:

**Classification**

```
test large_classify_one_word    ... bench:       4,460 ns/iter (+/- 243)
test large_classify_three_words ... bench:       8,289 ns/iter (+/- 1,543)
test small_classify_one_word    ... bench:         166 ns/iter (+/- 9)
test small_classify_three_words ... bench:         371 ns/iter (+/- 153)
```

**Training**

```
test train_large ... bench:  95,339,510 ns/iter (+/- 4,841,368)
test train_small ... bench:     125,168 ns/iter (+/- 9,914)
```
