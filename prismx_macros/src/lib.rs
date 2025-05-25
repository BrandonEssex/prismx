use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Widget)]
pub fn widget_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let gen = quote! {
        impl crate::renderable::Renderable for #name {
            fn draw<B: ratatui::backend::Backend>(
                &self,
                _frame: &mut ratatui::Frame<B>,
                _area: ratatui::layout::Rect,
            ) {
            }
        }
    };
    gen.into()
}
