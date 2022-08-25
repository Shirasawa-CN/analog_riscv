/*
作者:Shirasawa-CN(Cpointerz)
贡献者:
创建时间:2022/8/24
最后维护时间:2022/8/25
 */

mod realization;
use crate::realization::*;

use color_eyre::{eyre::eyre, Result};
use tracing::{error, info, instrument};
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

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


#[instrument]
fn return_err() -> Result<()> {
    Err(eyre!("Something went wrong"))
}

fn main() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    // 输出到文件中
    let file_appender = rolling::never("logs", "app.log");
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
