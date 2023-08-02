# BloomBox

BloomBox is a Bloom filter implementation written in Rust. A [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) is a space-efficient probabilistic data structure that is used to test whether an element is a member of a set. False positive matches are possible, but false negatives are not. The rate of false positives can be tuned.

## Status

Please note that BloomBox is currently in development and is not recommended for use in production environments. 

## Getting Started

To use BloomBox in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
bloombox = { git = "https://github.com/emersonmde/bloombox" }
```

## Usage

Here is a basic example of how to use BloomBox:

```rust
use bloombox::BloomFilter;

let mut filter = BloomFilter::new(100, 0.01);
filter.insert(&"item");
assert!(filter.contains(&"item"));
```

## Contributing

While this project is not actively maintained, contributions are welcome. If you find a bug or think of a new feature, please consider opening an issue or submitting a pull request. However, please understand that due to time constraints, pull requests might not receive immediate attention.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

