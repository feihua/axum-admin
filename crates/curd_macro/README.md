#简化rbatis orm 操作

## 1.添加Curd宏

```rust
#[derive(Clone, Debug, Serialize, Deserialize, Curd)]
pub struct Notice {
    pub id: Option<i64>,               //公告ID
    pub notice_title: String,          //公告标题
    pub notice_type: i8,               //公告类型（1:通知,2:公告）
    pub notice_content: String,        //公告内容
    pub status: i8,                    //公告状态（0:关闭,1:正常 ）
    pub remark: Option<String>,        //备注
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

```

**添加Curd宏会生成以下方法**

```text
//add notice
async fn add(rb: &RBatis, item: &T) -> rbatis::Result<Option<u64>>;

//delete notice
async fn delete(rb: &RBatis, ids: Vec<i64>) -> rbatis::Result<u64>;

//update notice
async fn update(rb: &RBatis, item: &T, id: i64) -> rbatis::Result<u64>;

//select by id
async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<T>>;

//update status
async fn update_status(rb: &RBatis, table_name: &str, ids: Vec<i64>, status: i8) -> rbatis::Result<u64>;
```

## 2.使用

```rust

pub struct NoticeService;

impl NoticeService {
    /// add notice
    pub async fn add_sys_notice(state: Arc<AppState>, mut item: NoticeReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let condition = value! {
            "notice_title": &item.notice_title,
        };
        if Notice::select_by_map(rb, condition).await?.len() > 0 {
            return Err(BusinessError("公告标题已存在"));
        }

        item.id = None;
        Notice::add(rb, &Notice::from(item)).await.map(|id| Ok(id.unwrap_or_default()))?
    }

    /// delete notice
    pub async fn delete_sys_notice(state: Arc<AppState>, item: DeleteNoticeReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        Notice::delete(rb, item.ids).await.map(|x| Ok(x))?
    }

    /// update notice
    pub async fn update_sys_notice(state: Arc<AppState>, item: NoticeReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let id = item.id;
        if id.is_none() {
            return Err(BusinessError("主键不能为空"));
        }

        if Notice::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(BusinessError("通知公告不存在"));
        }

        let condition = value! {
            "notice_title": &item.notice_title,
            "id !=":id
        };

        let list = Notice::select_by_map(rb, condition).await?;
        if list.len() > 0 {
            return Err(BusinessError("公告标题已存在"));
        }

        Notice::update(rb, &Notice::from(item), id.unwrap_or_default()).await.map(|x| Ok(x))?
    }

    /// update_status
    pub async fn update_sys_notice_status(state: Arc<AppState>, item: UpdateNoticeStatusReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        Notice::update_status(rb, "sys_notice", item.ids, item.status).await.map(|x| Ok(x))?
    }

    /// query_notice_detail
    pub async fn query_sys_notice_detail(state: Arc<AppState>, item: QueryNoticeDetailReq) -> Result<NoticeResp, AppError> {
        let rb = &state.batis;
        match Notice::select_by_id(rb, &item.id).await? {
            None => Err(BusinessError("通知公告不存在")),
            Some(x) => Ok(x.into()),
        }
    }
}

```

## 3.更多具体操作

更多具体操作请参考[axum-admin](https://github.com/feihua/axum-admin)

