# BloomBox

BloomBox is a serializable Bloom filter implementation using XXHash. A [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) is a space-efficient probabilistic data structure that is used to test whether an element is a member of a set with a predictable false positive rate. 


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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

