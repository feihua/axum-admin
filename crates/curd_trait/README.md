```rust
use rbatis::{RBatis, async_trait};

#[async_trait]
pub trait BaseCurd<T> {

    ///add
    async fn add(rb: &RBatis, item: &T) -> rbatis::Result<Option<u64>>;

    ///delete
    async fn delete(rb: &RBatis, ids: Vec<i64>) -> rbatis::Result<u64>;

    ///update
    async fn update(rb: &RBatis, item: &T, id: i64) -> rbatis::Result<u64>;

    ///update status
    async fn update_status(rb: &RBatis, table_name: &str, ids: Vec<i64>, status: i8) -> rbatis::Result<u64>;

    ///select by id
    async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<T>>;

    ///select by field
    ///select by int8 field
    async fn select_by_int8_field(rb: &RBatis, field: &str, value: &i8) -> rbatis::Result<Option<T>>;
    ///select by int32 field
    async fn select_by_int32_field(rb: &RBatis, field: &str, value: &i32) -> rbatis::Result<Option<T>>;
    ///select by int64 field
    async fn select_by_int64_field(rb: &RBatis, field: &str, value: &i64) -> rbatis::Result<Option<T>>;
    ///select by field
    async fn select_by_field(rb: &RBatis, field: &str, value: &str) -> rbatis::Result<Option<T>>;

}

```