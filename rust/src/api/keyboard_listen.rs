use std::sync::{Arc, Mutex};

use rdev::{listen, Event};

use std::thread;
use std::time::Duration;

use crate::frb_generated::StreamSink;

use super::syste::{LddEvent, LddKeyboardValue};

fn send_to_flutter(data: Vec<Event>, sink: StreamSink<LddKeyboardValue>) {
    let list: Vec<LddEvent> = data.into_iter().map(|v| v.into()).collect();
    if list.len() == 1 {
        let _ = sink.add(LddKeyboardValue::KeyboardValueV2(
            list.first().unwrap().clone(),
        ));
    } else {
        let _ = sink.add(LddKeyboardValue::ScanGunValueV2(list));
    }
}

pub fn start_listen(sink: StreamSink<LddKeyboardValue>) {
    let is_waiting = Arc::new(Mutex::new(false)); // 是否在执行等待时间中
    let can_send_event = Arc::new(Mutex::new(true)); // 是否可以发送数据到Flutter端
    let collected_data: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(vec![])); // 收集到的数据

    // 监听输入事件
    let is_waiting_clone = Arc::clone(&is_waiting);
    let can_send_event_clone = Arc::clone(&can_send_event);
    let collected_data_clone = Arc::clone(&collected_data);

    thread::spawn(move || {
        let _ = listen(move |input| {
            let mut is_waiting = is_waiting_clone.lock().unwrap();
            let mut can_send_event = can_send_event_clone.lock().unwrap();
            let mut collected_data = collected_data_clone.lock().unwrap();

            // 如果已经在等待时间中，则将输入数据收集起来
            if *is_waiting {
                collected_data.push(input);
                return;
            }

            // 否则，开始等待时间
            *is_waiting = true;
            *can_send_event = false;

            // 启动一个新的线程来处理延迟
            let is_waiting_clone_inner = Arc::clone(&is_waiting_clone);
            let can_send_event_clone_inner = Arc::clone(&can_send_event_clone);
            let collected_data_clone_inner = Arc::clone(&collected_data_clone);
            let input_clone = input.clone();
            let sink_clone = sink.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(150)); // 延迟100毫秒

                let mut is_waiting = is_waiting_clone_inner.lock().unwrap();
                let mut can_send_event = can_send_event_clone_inner.lock().unwrap();

                *is_waiting = false;
                *can_send_event = true;

                // 发送收集到的数据到Flutter端
                let collected_data = collected_data_clone_inner.lock().unwrap();
                send_to_flutter(collected_data.clone(), sink_clone);
            });

            // 在等待期间将当前输入数据收集起来
            collected_data.push(input_clone);
        });
    });
}
