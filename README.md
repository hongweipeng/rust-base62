# rust-base62
A simple library base62 encode/decode, no dependencies other libraries.

# Alphabet
It supports the standard `[0-9A-Za-z]` : [https://en.wikipedia.org/wiki/Base62](https://en.wikipedia.org/wiki/Base62)

```
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
```

# Example
```rust
use rust_base62;
fn main() {
    let plaintext = "hello";
    let ciphertext = rust_base62::encode(plaintext.as_bytes());
    let decode = rust_base62::decode(ciphertext.as_bytes()).unwrap();
    println!("cipher text: {}", ciphertext);
    println!("decode text: {}", String::from_utf8(decode).unwrap())
}
```
