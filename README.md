# Bayesic

A string matching library similar to a NaiveBayes classifier, but optimized for use cases where you have many possible matches.

This is especially useful if you have two large lists of names/titles/descriptions to match with each other.

# Performance

I'm doing this project both to learn about Rust and also because I want to improve the performance of https://github.com/mmmries/bayesic by making a rust extension.
I've added a few benchmarks for training and classifying on small and large data sets (ie 60k records).

Here are the current un-optimized results:

**Classification**

```
test large_classify_one_word    ... bench:  14,358,604 ns/iter (+/- 4,998,747)
test large_classify_three_words ... bench:  15,807,274 ns/iter (+/- 1,855,266)
test small_classify_one_word    ... bench:       1,578 ns/iter (+/- 221)
test small_classify_three_words ... bench:       2,533 ns/iter (+/- 490)
```

**Training**

```
test train_large ... bench: 100,110,190 ns/iter (+/- 6,489,334)
test train_small ... bench:     126,332 ns/iter (+/- 17,458)
```