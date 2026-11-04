use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, Ident, ItemFn, parse_macro_input, parse_quote,
    visit::{self, Visit},
};

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    let name = func.sig.ident.clone();
    let original_block = *func.block;

    let mut collector = BindingCollector::default();

    for arg in &func.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            collector.visit_pat(&pat_type.pat);
        }
    }

    let arg_names: Vec<Ident> = collector.names.into_iter().filter(|ident| ident != "state").collect();

    let log_fields = arg_names.iter().map(|ident| {
        quote! { #ident = ?#ident }
    });

    func.block = Box::new(parse_quote!({
    let __log_call_started = ::std::time::Instant::now();
        ::tracing::info!(
            function = stringify!(#name),
            #(#log_fields,)*
            "调用函数"
        );

        let __log_call_result = async {
            #original_block
        }
        .await;


        ::tracing::info!(
            function = stringify!(#name),
            elapsed_ms = __log_call_started.elapsed().as_millis() as u64,
            "函数执行完毕"
        );

        __log_call_result
    }));

    quote!(#func).into()
}

#[derive(Default)]
struct BindingCollector {
    names: Vec<Ident>,
}

impl<'ast> Visit<'ast> for BindingCollector {
    fn visit_pat_ident(&mut self, pat: &'ast syn::PatIdent) {
        self.names.push(pat.ident.clone());
        visit::visit_pat_ident(self, pat);
    }
}
