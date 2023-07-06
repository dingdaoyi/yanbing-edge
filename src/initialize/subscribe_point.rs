use std::sync::mpsc;
use futures::{FutureExt, StreamExt};
use protocol_core::event_bus::{get_pubsub_model, PointEvent, set_pharos_pub_sub_model};
use protocol_core::PharosPubSubModel;
use pharos::{ObserveConfig, Channel};
use crate::config::cache::get_protocol_store;

pub async fn init_subscribe_point() {
    let state= get_protocol_store().unwrap();
    let (sender, mut receiver) = std::sync::mpsc::channel();
    let res=state.init_protocol(sender);
    match res {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("初始化错误:{:?}",err);

        }
    };
    tokio::spawn(async move{
        for point_event in receiver {
            println!("接收到的数据:{:?}",point_event)
        }
    });
}


// 处理上报逻辑
fn handler_event(event: PointEvent) {
    tracing::info!("点位:{},值:{:?}",event.point_id,event.value)
}