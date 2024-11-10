#![feature(c_variadic)]

mod bt5api;

/*fn main() {
    unsafe {
        bt5api::iidx_io_set_loggers(Some(bt5api::log), Some(bt5api::log), Some(bt5api::log), Some(bt5api::log));
        bt5api::iidx_io_init(Some(bt5api::create_thread), Some(bt5api::join_thread), Some(bt5api::destroy_thread));

        let mut previous = 0;
        // Poll at a rate of 1000Hz
        loop {
            let current_time = std::time::Instant::now();
            bt5api::iidx_io_ep1_send();
            bt5api::iidx_io_ep2_recv();

            let keys = bt5api::iidx_io_ep2_get_keys();
            if keys != previous {
                println!("Keys: {:016b}", keys);
            }
            previous = keys;

            let elapsed = current_time.elapsed();
            if elapsed.as_micros() < 1000 {
                std::thread::sleep(std::time::Duration::from_micros(1000) - elapsed);
            }
        }
    }
}*/

use robusta_jni::bridge;

#[bridge]
mod jni {
    use robusta_jni::convert::Signature;
    use super::bt5api;

    #[derive(Signature)]
    #[package(fr.adamaq01.bt5api)]
    struct BToolsAPI;

    impl BToolsAPI {
        pub extern "jni" fn init() {
            unsafe {
                bt5api::iidx_io_set_loggers(Some(bt5api::log), Some(bt5api::log), Some(bt5api::log), Some(bt5api::log));
                bt5api::iidx_io_init(Some(bt5api::create_thread), Some(bt5api::join_thread), Some(bt5api::destroy_thread));
            }
        }

        pub extern "jni" fn poll() {
            unsafe {
                bt5api::iidx_io_ep1_send();
                bt5api::iidx_io_ep2_recv();
            }
        }

        pub extern "jni" fn keys() -> i16 {
            unsafe {
                bt5api::iidx_io_ep2_get_keys() as i16
            }
        }

        pub extern "jni" fn panel() -> i8 {
            unsafe {
                bt5api::iidx_io_ep2_get_panel() as i8
            }
        }

        pub extern "jni" fn sys() -> i8 {
            unsafe {
                bt5api::iidx_io_ep2_get_sys() as i8
            }
        }

        pub extern "jni" fn turntable(player: u8) -> i8 {
            unsafe {
                bt5api::iidx_io_ep2_get_turntable(player) as i8
            }
        }

        pub extern "jni" fn slider(slider: u8) -> i8 {
            unsafe {
                bt5api::iidx_io_ep2_get_slider(slider) as i8
            }
        }
    }
}
