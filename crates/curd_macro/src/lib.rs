use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
#[proc_macro_derive(Curd)]
pub fn derive_curd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generated_code = quote! {
    use curd_trait::BaseCurd;
    use rbatis::async_trait;

    #[async_trait]
    impl BaseCurd<#name> for #name {

     async fn add(rb: &RBatis, item: & #name) -> rbatis::Result<Option<u64>> {
        #name::insert(rb, item).await.map(|x| Ok(x.last_insert_id.as_u64())).map_err(|e| {
            error!("Database error insert into #name: {:?}", e);
            e
        })?
    }

     async fn delete(rb: &RBatis, ids: Vec<i64>) -> rbatis::Result<u64> {
        #name::delete_by_map(rb, value! {"id": ids}).await.map(|x| Ok(x.rows_affected)).map_err(|e| {
            error!("Database error delete #name: {:?}", e);
            e
        })?
    }

     async fn update(rb: &RBatis, item: &#name, id: i64) -> rbatis::Result<u64> {
        #name::update_by_map(rb, item, value! {"id": id}).await.map(|x| Ok(x.rows_affected)).map_err(|e| {
            error!("Database error update #name: {:?}", e);
            e
        })?
    }

     async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<#name>> {
        Ok(#name::select_by_map(rb, value! {"id": id}).await?.first().cloned())
    }

     async fn select_by_int8_field(rb: &RBatis, field: &str, value: &i8) -> rbatis::Result<Option<#name>> {
        Ok(#name::select_by_map(rb, value! {field: value}).await?.first().cloned())
    }

     async fn select_by_int32_field(rb: &RBatis, field: &str, value: &i32) -> rbatis::Result<Option<#name>> {
        Ok(#name::select_by_map(rb, value! {field: value}).await?.first().cloned())
    }

     async fn select_by_int64_field(rb: &RBatis, field: &str, value: &i64) -> rbatis::Result<Option<#name>> {
        Ok(#name::select_by_map(rb, value! {field: value}).await?.first().cloned())
    }

     async fn select_by_field(rb: &RBatis, field: &str, value: &str) -> rbatis::Result<Option<#name>> {
        Ok(#name::select_by_map(rb, value! {field: value}).await?.first().cloned())
    }
     async fn update_status(rb: &RBatis, table_name: &str, ids: Vec<i64>, status: i8) -> rbatis::Result<u64> {

        let update_sql = format!(
            "update {} set status = ? ,update_time = ? where id in ({})",
            table_name,
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(status), value!(DateTime::now())];
        param.extend(ids.iter().map(|&id| value!(id)));

        rb.exec(&update_sql, param).await.map(|x| Ok(x.rows_affected)).map_err(|e| {
            error!("Database error update_status #name: {:?}", e);
            e
        })?
    }
    }

    };
    TokenStream::from(generated_code)
}
