#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]


use josh_hates_closures::not_a_closure;

fn main() {
    let _ = #[not_a_closure] 32;
}
