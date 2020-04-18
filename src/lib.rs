use proc_macro::TokenStream;

use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn not_a_closure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let orig = item.clone();
    let expr = parse_macro_input!(item as syn::Expr);

    let output = if let syn::Expr::Closure(expr_closure) = expr {
        syn::Error::new(expr_closure.span(), "DON'T NEED NO STINKEN LAMBDA").to_compile_error().into()
    } else {
        orig
    };

    output.into()
}
