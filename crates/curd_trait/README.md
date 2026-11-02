```rust
use rbatis::{RBatis, async_trait};

/// Curd trait for a struct.
#[async_trait]
pub trait BaseCurd<T> {
    /// Insert a new item into the database.
    async fn add(rb: &RBatis, item: &T) -> rbatis::Result<Option<u64>>;

    /// Delete items from the database by their IDs.
    async fn delete(rb: &RBatis, ids: Vec<i64>) -> rbatis::Result<u64>;

    /// Update an item in the database by its ID.
    async fn update(rb: &RBatis, item: &T, id: i64) -> rbatis::Result<u64>;

    /// Update the status of items in the database by their IDs.
    async fn update_status(rb: &RBatis, table_name: &str, ids: Vec<i64>, status: i8) -> rbatis::Result<u64>;

    /// Select an item from the database by its ID.
    async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<T>>;

    /// Select an item from the database by a specific field and value.
    async fn select_by_int8_field(rb: &RBatis, field: &str, value: &i8) -> rbatis::Result<Option<T>>;
    /// Select an item from the database by a specific field and value.
    async fn select_by_int32_field(rb: &RBatis, field: &str, value: &i32) -> rbatis::Result<Option<T>>;
    /// Select an item from the database by a specific field and value.
    async fn select_by_int64_field(rb: &RBatis, field: &str, value: &i64) -> rbatis::Result<Option<T>>;
    /// Select an item from the database by a specific field and value.
    async fn select_by_field(rb: &RBatis, field: &str, value: &str) -> rbatis::Result<Option<T>>;

}

```