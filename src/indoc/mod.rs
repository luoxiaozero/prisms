mod unindent;

use unindent::do_unindent;

pub fn lit_indoc<'a>(code: &'a str) -> String {
    do_unindent(&code, false)
}
