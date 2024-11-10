/* automatically generated by rust-bindgen 0.70.1 */
/* modified manually after */

pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub type log_formatter_t = ::std::option::Option<
    unsafe extern "C" fn(
        module: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ),
>;
pub type thread_create_t = ::std::option::Option<
    unsafe extern "C" fn(
        proc_: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        ctx: *mut ::std::os::raw::c_void,
        stack_sz: u32,
        priority: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int,
>;
pub type thread_join_t = ::std::option::Option<
    unsafe extern "C" fn(thread_id: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int),
>;
pub type thread_destroy_t =
    ::std::option::Option<unsafe extern "C" fn(thread_id: ::std::os::raw::c_int)>;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_0: eam_io_keypad_scan_code = 0;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_1: eam_io_keypad_scan_code = 1;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_4: eam_io_keypad_scan_code = 2;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_7: eam_io_keypad_scan_code = 3;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_00: eam_io_keypad_scan_code = 4;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_2: eam_io_keypad_scan_code = 5;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_5: eam_io_keypad_scan_code = 6;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_8: eam_io_keypad_scan_code = 7;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_DECIMAL: eam_io_keypad_scan_code = 8;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_3: eam_io_keypad_scan_code = 9;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_6: eam_io_keypad_scan_code = 10;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_9: eam_io_keypad_scan_code = 11;
pub const eam_io_keypad_scan_code_EAM_IO_KEYPAD_COUNT: eam_io_keypad_scan_code = 12;
pub type eam_io_keypad_scan_code = ::std::os::raw::c_uint;
pub const eam_io_sensor_state_EAM_IO_SENSOR_FRONT: eam_io_sensor_state = 0;
pub const eam_io_sensor_state_EAM_IO_SENSOR_BACK: eam_io_sensor_state = 1;
pub type eam_io_sensor_state = ::std::os::raw::c_uint;
pub const eam_io_card_slot_cmd_EAM_IO_CARD_SLOT_CMD_CLOSE: eam_io_card_slot_cmd = 0;
pub const eam_io_card_slot_cmd_EAM_IO_CARD_SLOT_CMD_OPEN: eam_io_card_slot_cmd = 1;
pub const eam_io_card_slot_cmd_EAM_IO_CARD_SLOT_CMD_EJECT: eam_io_card_slot_cmd = 2;
pub const eam_io_card_slot_cmd_EAM_IO_CARD_SLOT_CMD_READ: eam_io_card_slot_cmd = 3;
pub type eam_io_card_slot_cmd = ::std::os::raw::c_uint;
pub const eam_io_read_card_result_EAM_IO_CARD_NONE: eam_io_read_card_result = 0;
pub const eam_io_read_card_result_EAM_IO_CARD_ISO15696: eam_io_read_card_result = 1;
pub const eam_io_read_card_result_EAM_IO_CARD_FELICA: eam_io_read_card_result = 2;
pub type eam_io_read_card_result = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct eam_io_config_api {
    _unused: [u8; 0],
}
#[link(name = "eamio", kind = "raw-dylib")]
extern "C" {
    pub fn eam_io_set_loggers(
        misc: log_formatter_t,
        info: log_formatter_t,
        warning: log_formatter_t,
        fatal: log_formatter_t,
    );

    pub fn eam_io_init(
        thread_create: thread_create_t,
        thread_join: thread_join_t,
        thread_destroy: thread_destroy_t,
    ) -> bool;

    pub fn eam_io_fini();

    pub fn eam_io_get_keypad_state(unit_no: u8) -> u16;

    pub fn eam_io_get_sensor_state(unit_no: u8) -> u8;

    pub fn eam_io_read_card(unit_no: u8, card_id: *mut u8, nbytes: u8) -> u8;

    pub fn eam_io_card_slot_cmd(unit_no: u8, cmd: u8) -> bool;

    pub fn eam_io_poll(unit_no: u8) -> bool;

    pub fn eam_io_get_config_api() -> *const eam_io_config_api;
}
pub const iidx_io_sys_bit_IIDX_IO_SYS_TEST: iidx_io_sys_bit = 0;
pub const iidx_io_sys_bit_IIDX_IO_SYS_SERVICE: iidx_io_sys_bit = 1;
pub const iidx_io_sys_bit_IIDX_IO_SYS_COIN: iidx_io_sys_bit = 2;
pub type iidx_io_sys_bit = ::std::os::raw::c_uint;
pub const iidx_io_panel_bit_IIDX_IO_PANEL_P1_START: iidx_io_panel_bit = 0;
pub const iidx_io_panel_bit_IIDX_IO_PANEL_P2_START: iidx_io_panel_bit = 1;
pub const iidx_io_panel_bit_IIDX_IO_PANEL_VEFX: iidx_io_panel_bit = 2;
pub const iidx_io_panel_bit_IIDX_IO_PANEL_EFFECT: iidx_io_panel_bit = 3;
pub type iidx_io_panel_bit = ::std::os::raw::c_uint;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_1: iidx_io_key_bit = 0;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_2: iidx_io_key_bit = 1;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_3: iidx_io_key_bit = 2;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_4: iidx_io_key_bit = 3;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_5: iidx_io_key_bit = 4;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_6: iidx_io_key_bit = 5;
pub const iidx_io_key_bit_IIDX_IO_KEY_P1_7: iidx_io_key_bit = 6;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_1: iidx_io_key_bit = 7;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_2: iidx_io_key_bit = 8;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_3: iidx_io_key_bit = 9;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_4: iidx_io_key_bit = 10;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_5: iidx_io_key_bit = 11;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_6: iidx_io_key_bit = 12;
pub const iidx_io_key_bit_IIDX_IO_KEY_P2_7: iidx_io_key_bit = 13;
pub type iidx_io_key_bit = ::std::os::raw::c_uint;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_1: iidx_io_deck_light = 0;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_2: iidx_io_deck_light = 1;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_3: iidx_io_deck_light = 2;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_4: iidx_io_deck_light = 3;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_5: iidx_io_deck_light = 4;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_6: iidx_io_deck_light = 5;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P1_7: iidx_io_deck_light = 6;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_1: iidx_io_deck_light = 8;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_2: iidx_io_deck_light = 9;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_3: iidx_io_deck_light = 10;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_4: iidx_io_deck_light = 11;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_5: iidx_io_deck_light = 12;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_6: iidx_io_deck_light = 13;
pub const iidx_io_deck_light_IIDX_IO_DECK_LIGHT_P2_7: iidx_io_deck_light = 14;
pub type iidx_io_deck_light = ::std::os::raw::c_uint;
pub const iidx_io_panel_light_IIDX_IO_PANEL_LIGHT_P1_START: iidx_io_panel_light = 0;
pub const iidx_io_panel_light_IIDX_IO_PANEL_LIGHT_P2_START: iidx_io_panel_light = 1;
pub const iidx_io_panel_light_IIDX_IO_PANEL_LIGHT_VEFX: iidx_io_panel_light = 2;
pub const iidx_io_panel_light_IIDX_IO_PANEL_LIGHT_EFFECT: iidx_io_panel_light = 3;
pub type iidx_io_panel_light = ::std::os::raw::c_uint;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_LEFT_BLUE: iidx_io_top_lamp = 0;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_LEFT_GREEN: iidx_io_top_lamp = 1;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_LEFT_YELLOW: iidx_io_top_lamp = 2;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_LEFT_RED: iidx_io_top_lamp = 3;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_RIGHT_BLUE: iidx_io_top_lamp = 4;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_RIGHT_GREEN: iidx_io_top_lamp = 5;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_RIGHT_YELLOW: iidx_io_top_lamp = 6;
pub const iidx_io_top_lamp_IIDX_IO_TOP_LAMP_RIGHT_RED: iidx_io_top_lamp = 7;
pub type iidx_io_top_lamp = ::std::os::raw::c_uint;
#[link(name = "iidxio", kind = "raw-dylib")]
extern "C" {
    pub fn iidx_io_set_loggers(
        misc: log_formatter_t,
        info: log_formatter_t,
        warning: log_formatter_t,
        fatal: log_formatter_t,
    );

    pub fn iidx_io_init(
        thread_create: thread_create_t,
        thread_join: thread_join_t,
        thread_destroy: thread_destroy_t,
    ) -> bool;

    pub fn iidx_io_fini();

    pub fn iidx_io_ep1_set_deck_lights(deck_lights: u16);

    pub fn iidx_io_ep1_set_panel_lights(panel_lights: u8);

    pub fn iidx_io_ep1_set_top_lamps(top_lamps: u8);

    pub fn iidx_io_ep1_set_top_neons(top_neons: bool);

    pub fn iidx_io_ep1_send() -> bool;

    pub fn iidx_io_ep2_recv() -> bool;

    pub fn iidx_io_ep2_get_turntable(player_no: u8) -> u8;

    pub fn iidx_io_ep2_get_slider(slider_no: u8) -> u8;

    pub fn iidx_io_ep2_get_sys() -> u8;

    pub fn iidx_io_ep2_get_panel() -> u8;

    pub fn iidx_io_ep2_get_keys() -> u16;

    pub fn iidx_io_ep3_write_16seg(text: *const ::std::os::raw::c_char) -> bool;
}
