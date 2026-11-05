use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Path, Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct RouteEntry {
    path: LitStr,
    method: Ident,
    handler: Path,
}

struct RouteList {
    entries: Vec<RouteEntry>,
}

impl Parse for RouteList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            let path: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;

            let method: Ident = input.parse()?;

            let handler_content;
            parenthesized!(handler_content in input);
            let handler: Path = handler_content.parse()?;

            if !handler_content.is_empty() {
                return Err(handler_content.error("括号中只允许填写一个 handler 路径"));
            }

            entries.push(RouteEntry { path, method, handler });

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        if entries.is_empty() {
            return Err(syn::Error::new(proc_macro2::Span::call_site(), "routes! 至少需要一个路由"));
        }

        Ok(RouteList { entries })
    }
}

/// 函数式过程宏：将路由声明展开为 Axum Router。
///
/// 用法：
/// routes! {
///     "/path" => post(handler),
/// }
#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let RouteList { entries, .. } = parse_macro_input!(input as RouteList);

    let route_calls = entries.iter().map(|entry| {
        let path = &entry.path;
        let method = &entry.method;
        let handler = &entry.handler;

        quote! {
            .route(#path, ::axum::routing::#method(#handler))
        }
    });

    quote! {
        ::axum::Router::new()
            #(#route_calls)*
    }
    .into()
}
