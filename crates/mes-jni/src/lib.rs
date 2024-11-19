mod utils;

use jni::{
    objects::{JByteArray, JClass, JFloatArray},
    JNIEnv,
};
use mes_core::{mappers::MapperChip, Nes};
use utils::{MutUnwrap, RefUnwrap};

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_init(
    _env: JNIEnv<'static>,
    _class: JClass,
) -> *mut Nes {
    let mapper = MapperChip::mock();
    let nes = Nes::with_mapper(mapper);
    Box::into_raw(nes.into())
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_reset(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().reset();
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_free(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    unsafe { drop(Box::from_raw(nes)) };
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_setCartridge(
    mut env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
    cartridge: JByteArray,
) {
    let len = env
        .get_array_length(&cartridge)
        .expect("Failed to get ROM length");

    let mut buffer = vec![0; len as usize];

    env.get_byte_array_region(&cartridge, 0, &mut buffer)
        .expect("Failed to load ROM cartridge into buffer");

    let ptr = buffer.as_ptr() as *const u8;
    let buffer = unsafe { std::slice::from_raw_parts(ptr, len as usize) };

    if let Err(err) = nes.unwrap_mut().set_cartridge(buffer) {
        env.throw(err.to_string()).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_stepFrame(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().step_frame();
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_stepVblank(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().step_vblank();
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_getAudioBuffer<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
    nes: *const Nes,
) -> JFloatArray<'local> {
    let buffer = nes.unwrap_ref().get_audio_buffer();

    let float_arr = env
        .new_float_array(buffer.len() as i32)
        .expect("Failed to get audio buffer length");

    env.set_float_array_region(&float_arr, 0, &buffer)
        .expect("Failed to load audio buffer");

    float_arr
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_clearAudioBuffer(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().clear_audio_buffer();
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_setControllerState(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
    id: usize,
    state: u8,
) {
    nes.unwrap_mut().set_controller_state(id, state);
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_Nes_getFrameBuffer<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
    nes: *const Nes,
) -> JByteArray<'local> {
    let buffer = nes.unwrap_ref().get_frame_buffer();
    let buffer = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const i8, buffer.len()) };

    let byte_arr = env
        .new_byte_array(buffer.len() as i32)
        .expect("Failed to get frame buffer length");

    env.set_byte_array_region(&byte_arr, 0, buffer)
        .expect("Failed to load frame buffer");

    byte_arr
}
