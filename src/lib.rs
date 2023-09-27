use boa_engine::{Context, Source};
use std::cell::RefCell;

const PRISM_JS: &str = include_str!("../prism/prism.js");
pub const PRISM_CSS: &str = include_str!("../prism/prism.css");

thread_local! {
    static CONTEXT: RefCell<Context<'static>> = {
        let mut context = Context::default();
        context
            .eval(Source::from_bytes(PRISM_JS))
            .expect("Error initializing PRISM_JS");
        RefCell::new(context)
    };
}

fn with_context<T>(f: impl FnOnce(&mut Context<'_>) -> T) -> T {
    CONTEXT.with(|context| {
        let mut context = context.borrow_mut();
        f(&mut *context)
    })
}

/// `text`: the code to be highlighted
///
/// `grammar`: the name of the prism.js language definition in the context
///
/// `language`: the name of the language definition passed to grammar
pub fn highlight(text: &str, grammar: &str, language: &str) -> String {
    with_context(|context| {
        context
            .global_object()
            .set("text", text, true, context)
            .unwrap();
        context
            .global_object()
            .set("language", language, true, context)
            .unwrap();

        let src: String = format!("Prism.highlight(text, {grammar}, language)");
        let src = Source::from_bytes(&src);
        let html = context.eval(src).expect("highlight execution failed");

        html.to_string(context)
            .expect("highlight execution results return an error")
            .to_std_string_escaped()
    })
}

#[test]
fn highlight_test() {
    let js_code = r#"let two = 1 + 1;"#;
    let html = highlight(js_code, "Prism.languages.javascript", "javascript");
    assert_eq!(
        html,
        r#"<span class="token keyword">let</span> two <span class="token operator">=</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>"#
    );
}
