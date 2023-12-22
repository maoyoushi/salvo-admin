use once_cell::sync::Lazy;
use rbatis::RBatis;
use redis::Client;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};
use tracing;

mod controller;
mod entity;
mod mapper;
mod model;
mod router;
mod service;
mod utils;

pub static GLOBAL_DB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

pub static GLOBAL_REDIS: Lazy<Client> =
    Lazy::new(|| Client::open("redis://127.0.0.1/").expect("连接redis失败"));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 连接数据库
    utils::mysql::init_db().await;
    tracing::info!("数据库连接成功");

    // 连接redis
    GLOBAL_REDIS.get_connection().expect("连接redis失败");
    tracing::info!("redis连接成功");

    let service = router::init_service();

    Server::new(TcpListener::new("0.0.0.0:8090").bind().await)
        .serve(service)
        .await;
}
