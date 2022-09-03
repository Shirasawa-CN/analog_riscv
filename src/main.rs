/*
作者:Shirasawa-CN(Cpointerz)
贡献者:
创建时间:2022/8/24
最后维护时间:2022/8/25
 */

use chrono::prelude::*;
use color_eyre::{eyre::eyre, Result};
use tracing::{error, info, instrument};
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, Registry, util::SubscriberInitExt,
};

use crate::realization::*;

mod realization;
/*
如果要使用log反馈测试，请在下面User完善代码
 */
//可修改范围的开始
struct User;

impl Default for User{
    fn default() -> Self {
        Self
    }
}

impl User{
    fn new(self) -> Self{
        Self
    }
}
//可修改范围的结束
/*
下面的main中还有可修改的位置
 */

fn main() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    // 输出到文件中
    let time: DateTime<Local> = Local::now();
    // let time: DateTime<Utc> = Utc::now();
    let file_name = format!("{}-{}-{}-{}-{}.log", time.year(), time.month(), time.day(), time.hour(), time.minute());
    let file_appender = rolling::never("logs", file_name);
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    // 安裝 color-eyre 的 panic 处理句柄
    color_eyre::install()?;

    //可修改的位置
    let user = User::default();
    //let user = User::new();

    Ok(())
}
