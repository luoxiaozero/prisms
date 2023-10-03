# prisms

[Prism](https://prismjs.com) at compile time for rust

## Usage

```toml
[dependencies]
prisms = { git = "https://github.com/luoxiaozero/prisms" }
```

```rust
use prisms::highlight_str;

fn main() {
    let html = highlight_str!("let two = 1 + 1;", "javascript");
    assert_eq!(
        html,
        r#"<span class="token keyword">let</span> two <span class="token operator">=</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>"#
    );
}

```

## Resources

[prism-js](https://github.com/FraserLee/prism-rs)

[indoc](https://github.com/dtolnay/indoc)
