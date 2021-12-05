extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn main(item: TokenStream) -> TokenStream {
    let day = item.to_string();
    format!(
        r#"
    fn main() {{
        use aoc::AoCSolution;
        Day{day}::run({day});
    }}
    "#,
        day = day
    )
    .parse()
    .unwrap()
}
