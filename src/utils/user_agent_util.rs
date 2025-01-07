use regex::Regex;

#[derive(Debug, Clone)]
pub struct UserAgentUtil {
    pub platform: String,       //平台信息
    pub os: String,             //操作系统信息
    pub arch: String,           //体系结构信息
    pub engine: String,         //渲染引擎信息
    pub engine_details: String, //渲染引擎详细信息
    pub browser: String,        //浏览器名称
    pub version: String,        //浏览器版本
    pub extra: String,          //其他信息（可选）
}

impl UserAgentUtil {
    pub fn new(user_agent: &str) -> Self {
        let mut parse = UserAgentUtil {
            platform: "".to_string(),
            os: "".to_string(),
            arch: "".to_string(),
            engine: "".to_string(),
            engine_details: "".to_string(),
            browser: "".to_string(),
            version: "".to_string(),
            extra: "".to_string(),
        };
        // 定义正则表达式来匹配并提取信息
        let re = Regex::new(
            r"(?x)
        Mozilla/5.0 \s*                      # Mozilla标识
        \((?P<platform>[^;]+); \s*            # 平台信息
        (?P<os>[^;]+); \s*                    # 操作系统信息
        (?P<arch>[^\)]+)\) \s*                # 体系结构信息
        (?P<engine>[^\s]+) \s*                # 渲染引擎信息
        \((?P<engine_details>[^\)]+)\) \s*    # 渲染引擎详细信息
        (?P<browser>[^\s/]+)/                 # 浏览器名称
        (?P<version>[^\s]+) \s*               # 浏览器版本
        (?P<extra>[^\s]+)?                    # 其他信息（可选）
    ",
        )
        .unwrap();

        // 使用正则表达式进行匹配
        if let Some(captures) = re.captures(user_agent) {
            let platform = captures.name("platform").unwrap().as_str();
            let os = captures.name("os").unwrap().as_str();
            let arch = captures.name("arch").unwrap().as_str();
            let engine = captures.name("engine").unwrap().as_str();
            let engine_details = captures.name("engine_details").unwrap().as_str();
            let browser = captures.name("browser").unwrap().as_str();
            let version = captures.name("version").unwrap().as_str();
            let extra = captures.name("extra").map_or("", |m| m.as_str());

            parse.platform = platform.to_string();
            parse.os = os.to_string();
            parse.arch = arch.to_string();
            parse.engine = engine.to_string();
            parse.engine_details = engine_details.to_string();
            parse.browser = browser.to_string();
            parse.version = version.to_string();
            parse.extra = extra.to_string();
        }

        parse
    }
}
