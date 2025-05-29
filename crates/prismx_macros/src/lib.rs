use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl prismx::render::traits::Renderable for #name {
            fn render_frame<B: ratatui::prelude::Backend>(
                &mut self,
                _f: &mut ratatui::Frame<B>,
                _area: ratatui::prelude::Rect,
            ) { }

            fn tick(&mut self) { }
        }

        impl prismx::render::traits::Widget for #name {}
    };

    TokenStream::from(expanded)
}
