mod utils;

use jni::{
    objects::{JByteArray, JClass, JFloatArray, JIntArray},
    JNIEnv,
};
use mes_core::{mappers::MapperChip, ppu, Nes};
use utils::{MutUnwrap, RefUnwrap};

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_init(
    _env: JNIEnv<'static>,
    _class: JClass,
) -> *mut Nes {
    let mapper = MapperChip::mock();
    let nes = Nes::with_mapper(mapper);
    Box::into_raw(nes.into())
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_reset(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().reset();
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_free(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    unsafe { drop(Box::from_raw(nes)) };
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_setCartridge(
    mut env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
    cartridge: JByteArray,
) {
    let buffer = env
        .convert_byte_array(cartridge)
        .expect("Failed to load ROM");

    if let Err(err) = nes.unwrap_mut().set_cartridge(&buffer) {
        env.throw(err.to_string()).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_stepFrame(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().step_frame();
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_stepVblank(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().step_vblank();
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_fillAudioBuffer(
    env: JNIEnv<'static>,
    _class: JClass,
    nes: *const Nes,
    float_arr: JFloatArray<'static>,
) {
    let buffer = nes.unwrap_ref().get_audio_buffer();

    env.set_float_array_region(&float_arr, 0, &buffer)
        .expect("Failed to load audio buffer");
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_clearAudioBuffer(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().clear_audio_buffer();
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_setControllerState(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
    id: usize,
    state: u8,
) {
    nes.unwrap_mut().set_controller_state(id, state);
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_fillFrameBuffer(
    mut env: JNIEnv<'static>,
    _class: JClass,
    nes: *const Nes,
    int_arr: JIntArray<'static>,
) {
    let buffer = nes.unwrap_ref().get_frame_buffer();

    let mut elements = unsafe {
        env.get_array_elements(&int_arr, jni::objects::ReleaseMode::CopyBack)
            .expect("Failed to get frame buffer")
    };

    for (i, pixel) in buffer.iter().enumerate() {
        let color_index = *pixel as usize;
        let a = 255u32;
        let r = ppu::COLOR_PALETTE[color_index] as u32;
        let g = ppu::COLOR_PALETTE[color_index + 1] as u32;
        let b = ppu::COLOR_PALETTE[color_index + 2] as u32;
        elements[i] = (a << 24 | r << 16 | g << 8 | b) as i32;
    }
}
