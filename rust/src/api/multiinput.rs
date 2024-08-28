use std::{collections::HashMap, time::Duration};

use crate::{
    api::{
        entitys::{LddKeyId, LddKeyboard, LddRawEvent},
        logger::init_logger,
    },
    frb_generated::{StreamSink, FLUTTER_RUST_BRIDGE_HANDLER},
};
use flutter_rust_bridge::frb;
use log::debug;
use multiinput::{KeyId, KeyboardDisplayInfo, RawEvent, RawInputManager};

unsafe impl Sync for LddKeyboardManager {}

///键盘管理器
pub struct LddKeyboardManager {
    manager: RawInputManager,
    print_debug: Option<bool>,
    gun_add_end_return_key: Option<bool>
}

impl LddKeyboardManager {

    ///打印日志
    fn print_debug_log(&self,msg: String) {
        let p = self.print_debug.map_or(true, 
        |v|v);
        if p {
            debug!("{:?}",msg);
        }
    }

    //初始化键盘管理器
    #[frb(sync)]
    pub fn new(print_debug:Option<bool>,gun_add_end_return_key: Option<bool>) -> LddKeyboardManager {
        init_logger();
        
        let p = print_debug.map_or(true, |v|v);
        if p {
            debug!("初始化成功");
        }
    
        LddKeyboardManager {
            manager: RawInputManager::new().unwrap(),
            print_debug: print_debug,
            gun_add_end_return_key: gun_add_end_return_key
        }
    }

    ///注册键盘监听
    #[frb(sync)]
    pub fn register(&mut self) {
        self.print_debug_log("✔设置键盘监听事件成功".to_owned());
        self.manager
            .register_devices(multiinput::DeviceType::Keyboards);
    }

    ///读取键盘类型的设备信息
    #[frb(sync)]
    pub fn get_ldd_keyboard_list(&mut self) -> Vec<LddKeyboard> {
        let devices = self.manager.get_device_list();
        self.print_debug_log("读取键盘设备列表成功".to_owned());
        devices
            .keyboards
            .into_iter()
            .map(|item| item.into())
            .collect()
    }

    ///监听梁典典键盘数据
    /// [gun_device] - 扫码枪设备，如果不传，则不判断

    pub fn listen_ldd_keyboard_event(
        self,
        gun_device: Option<LddKeyboard>,
        sink: StreamSink<LddRawEvent>,
    ) {

        let printer_debug = self.print_debug.clone().map_or(true, |v|v);
        let add_return_key = self.gun_add_end_return_key.clone().map_or(true, |v|v);
        if printer_debug {
            debug!("开启监听子线程：：：：监听已开始。");
        }      
        let mut manager = self.manager;
        let mut scanner_buffers: HashMap<usize, Vec<LddKeyId>> = HashMap::new(); // 用于存储每个扫码枪的输入
        FLUTTER_RUST_BRIDGE_HANDLER
            .thread_pool()
            .0
            .execute(move || {
                loop {
                    if let Some(event) = manager.get_event() {
                        let keyboards = manager.get_device_list().keyboards;
                        match event {
                            RawEvent::KeyboardEvent(hid, keyid, state) => {
                                let device_item = keyboards.get(hid).unwrap(); //获取设备
                                let device = LddKeyboard::from_display_info(device_item);

                                //判断是不是扫码枪
                                let is_gun =
                                    gun_device.clone().map_or(false, |e| e.name == device.name);
                                if is_gun {
                                    //todo 组装字符串
                                    // 获取扫码枪对应的输入缓存
                                    let buffer = scanner_buffers
                                        .entry(hid)
                                        .or_insert_with(|| Vec::<LddKeyId>::new());

                                    let key_id_clone = keyid.clone();
                                    // 根据按键状态处理输入
                                    match state {
                                        multiinput::State::Pressed => {
                                            // 组装字符串
                                            let ldd_key: LddKeyId = keyid.into();

                                            //添加到缓存区域
                                           
                                        
                                            if  key_id_clone == KeyId::Return {
                                                if add_return_key {
                                                    buffer.push(ldd_key);
                                                }
                                            }else{
                                                buffer.push(ldd_key);
                                            }

                                            



                                            // 检查是否是回车键
                                            match key_id_clone {
                                                KeyId::Return => {
                                                    //发送
                                                    let _ = sink.add(LddRawEvent::ScanGunEvent(
                                                        buffer.to_vec(),
                                                        device,
                                                    ));
                                                    buffer.clear();
                                                }
                                                _ => {}
                                            };
                                        }
                                        multiinput::State::Released => {}
                                    };
                                } else {
                                    let ldd_event = LddRawEvent::LddKeyboardEvent(
                                        hid,
                                        keyid.into(),
                                        state.into(),
                                        device,
                                    );

                                    if printer_debug {
                                        debug!("{:?}", ldd_event);
                                    }
                                   
                    
                                    let _ = sink.add(ldd_event); //回调到flutter端
                                }
                            }
                            _ => {}
                        }
                    } else {
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
            });
    }
}

impl LddKeyboard {
    fn from_display_info(item: &KeyboardDisplayInfo) -> LddKeyboard {
        LddKeyboard {
            name: item.name.clone(),
            serial: item.serial.clone(),
        }
    }
}
