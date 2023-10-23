mod indoc;

use boa_engine::{Context, Source};
use indoc::lit_indoc;
use litrs::StringLit;
use proc_macro2::*;
use quote::quote;
use std::cell::RefCell;

const PRISM_JS: &str = include_str!("../prism/prism.js");
const PRISM_CSS: &str = include_str!("../prism/prism.css");

#[proc_macro]
pub fn prism_css(_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! { #PRISM_CSS })
}

thread_local! {
    static CONTEXT: Result<RefCell<Context<'static>>, String> = {
        let mut context = Context::default();
        if let Err(err) = context
            .eval(Source::from_bytes(PRISM_JS))
            .map_err(|_| "Error initializing PRISM_JS".to_string()) {
                return Err(err);
            }

        Ok(RefCell::new(context))
    };
}

fn with_context<T>(f: impl FnOnce(&mut Context<'_>) -> T) -> Result<T, String> {
    CONTEXT.with(|context| match context {
        Ok(context) => {
            let mut context = context.borrow_mut();
            Ok(f(&mut *context))
        }
        Err(err) => Err(err.clone()),
    })
}

/// `text`: the code to be highlighted
///
/// `grammar`: the name of the prism.js language definition in the context
///
/// `language`: the name of the language definition passed to grammar
fn highlight(text: &str, grammar: &str, language: &str) -> Result<String, String> {
    with_context(|context| {
        context
            .global_object()
            .set("text", text, true, context)
            .map_err(|_| "Set text error".to_string())?;
        context
            .global_object()
            .set("language", language, true, context)
            .map_err(|_| "Set language error".to_string())?;

        let src: String = format!("Prism.highlight(text, {grammar}, language)");
        let src = Source::from_bytes(&src);
        let html = context
            .eval(src)
            .map_err(|_| "highlight execution failed".to_string())?;

        let html = html
            .to_string(context)
            .map_err(|_| "highlight execution results return an error".to_string())?;
        Ok(html.to_std_string_escaped())
    })?
}

#[proc_macro]
pub fn highlight_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream = TokenStream::from(token_stream).into_iter();
    let expanded =
        highlight_str_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
    proc_macro::TokenStream::from(expanded)
}

fn highlight_str_(
    mut token_stream: impl Iterator<Item = TokenTree>,
) -> Result<TokenStream, String> {
    let first = token_stream.next();
    let Some(TokenTree::Literal(code)) = first else {
        return Err("Expected only a string literal".to_string());
    };
    let code =
        StringLit::try_from(code).map_err(|err| format!("Expected a string literal: {}", err))?;

    let second = token_stream.next();
    let Some(TokenTree::Punct(punct)) = second else {
        return Err("You must put a comma `,` after the code string".to_string());
    };

    if punct.as_char() != ',' {
        return Err("You must put a comma `,` after the code string".to_string());
    }

    let third = token_stream.next();
    let Some(TokenTree::Literal(lang)) = third else {
        return Err("Expected only a string literal".to_string());
    };
    let lang =
        StringLit::try_from(lang).map_err(|err| format!("Expected a string literal: {}", err))?;

    let html = highlight(
        &lit_indoc(code.value()),
        &format!("Prism.languages.{}", lang.value()),
        lang.value(),
    )?;
    Ok(quote!(#html))
}

#[test]
fn highlight_test() {
    let js_code = r#"let two = 1 + 1;"#;
    let html = highlight(js_code, "Prism.languages.javascript", "javascript").unwrap();
    assert_eq!(
        html,
        r#"<span class="token keyword">let</span> two <span class="token operator">=</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>"#
    );
}
