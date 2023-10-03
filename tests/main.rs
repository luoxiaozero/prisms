use prisms::highlight_str;

#[test]
fn highlight_str_test() {
    let html = highlight_str!("let two = 1 + 1;", "javascript");
    assert_eq!(
        html,
        r#"<span class="token keyword">let</span> two <span class="token operator">=</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>"#
    );
}
