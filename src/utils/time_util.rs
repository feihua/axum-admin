use rbatis::rbdc::DateTime;

/*
 *时间转字符串
 *author：刘飞华
 *date：2025/01/02 14:38:11
 */
pub fn time_to_string(t: Option<DateTime>) -> String {
    match t {
        None => "".to_string(),
        Some(x) => x.format("YYYY-MM-DD hh:mm:ss"),
    }
}
