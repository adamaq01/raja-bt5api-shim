#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("../bindings.rs");

use std::{os::windows::io::AsRawHandle, sync::{LazyLock, Mutex}, thread::JoinHandle};
use slotmap::{new_key_type, KeyData, SlotMap};
use std::os::raw::{c_int, c_uint, c_void};
use thread_priority::*;

new_key_type! {
    struct ThreadKey;
}

impl ThreadKey {
    pub fn as_ffi(self) -> u32 {
        let ffi = self.0.as_ffi();
        let idx = (ffi & 0xffff) as u16 as u32;
        let version = (ffi >> 32) as u16 as u32;
        (version << 16) | idx
    }

    pub fn from_ffi(value: u32) -> Self {
        let idx = value & 0xff;
        let version = value >> 16;
        Self(KeyData::from_ffi((version as u64) << 32 | idx as u64))
    }
}

static THREADS: LazyLock<Mutex<SlotMap<ThreadKey, JoinHandle<c_int>>>> = LazyLock::new(|| Mutex::new(SlotMap::with_key()));

pub unsafe extern "C" fn create_thread(
    proc: Option<
        unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
    >,
    ctx: *mut c_void,
    stack_sz: u32,
    priority: c_uint,
) -> c_int {
    let Some(proc) = proc else {
        return -1;
    };

    let Ok(mut guard) = THREADS.lock() else {
        return -1;
    };

    struct PointerWrapper(*mut c_void);
    unsafe impl Send for PointerWrapper {}
    let ctx = PointerWrapper(ctx);

    let Ok(handle) = ThreadBuilder::default()
        .name(format!("raja-btapi-thread-{}", guard.len()))
        .stack_size(stack_sz as usize)
        .priority(ThreadPriority::Os(priority.try_into().unwrap_or(ThreadPriorityOsValue::default())))
        .spawn_careless(move || {
            let _ = &ctx;
            proc(ctx.0)
        }) else {
        return -1;
    };

    guard.insert(handle).as_ffi() as c_int
}

pub unsafe extern "C" fn join_thread(thread_id: c_int, result: *mut c_int) {
    let thread_id = ThreadKey::from_ffi(thread_id as u32);

    let Ok(mut guard) = THREADS.lock() else {
        return;
    };

    let Some(handle) = guard.remove(thread_id) else {
        return;
    };

    *result = handle.join().unwrap_or(-1);
}

// TODO: xd
pub unsafe extern "C" fn destroy_thread(thread_id: c_int) {
    let thread_id = ThreadKey::from_ffi(thread_id as u32);

    let Ok(mut guard) = THREADS.lock() else {
        return;
    };

    guard.remove(thread_id);
}

pub unsafe extern "C" fn log(
    module: *const ::std::os::raw::c_char,
    fmt: *const ::std::os::raw::c_char,
    mut args: ...
) {
    let module = std::ffi::CStr::from_ptr(module).to_string_lossy();
    let mut fmt = std::ffi::CStr::from_ptr(fmt).to_string_lossy();
    if !module.is_empty() {
        fmt = format!("[{}] {}\n", module, fmt).into();
    }
    libc::printf(fmt.as_ptr() as *const i8, args);
}
