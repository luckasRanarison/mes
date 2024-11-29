mod utils;

use jni::{
    objects::{JByteArray, JClass, JFloatArray, JIntArray, JObject, JString},
    JNIEnv,
};
use mes_core::{json::serialize_rom_header, mappers::MapperChip, ppu, Nes};
use utils::{MutUnwrap, RefUnwrap};

fn log_error(mut env: JNIEnv, tag: &str, message: &str) -> jni::errors::Result<()> {
    let log_class = env.find_class("android/util/Log")?;
    let j_tag = env.new_string(tag)?;
    let j_message = env.new_string(message)?;

    env.call_static_method(
        log_class,
        "e",
        "(Ljava/lang/String;Ljava/lang/String;)I",
        &[(&j_tag).into(), (&j_message).into()],
    )?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Rust_setPanicHook(
    env: JNIEnv<'static>,
    _class: JClass,
) {
    let jvm = env.get_java_vm().unwrap();

    std::panic::set_hook(Box::new(move |info| {
        let env = jvm.get_env().unwrap();
        log_error(env, "mes", info.to_string().as_str()).unwrap();
    }));
}

#[no_mangle]
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_serializeRomHeader<'local>(
    mut env: JNIEnv<'static>,
    _class: JClass,
    rom: JByteArray<'local>,
) -> JString<'local> {
    let bytes = env.convert_byte_array(rom).expect("Failed to load ROM");

    match serialize_rom_header(&bytes) {
        Ok(json) => env.new_string(json).unwrap(),
        Err(err) => env
            .throw(err.to_string())
            .map(|_| JObject::null().into())
            .unwrap(),
    }
}

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
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_step(
    _env: JNIEnv<'static>,
    _class: JClass,
    nes: *mut Nes,
) {
    nes.unwrap_mut().step();
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
pub extern "C" fn Java_dev_luckasranarison_mes_lib_Nes_stepVBlank(
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
) -> u32 {
    let buffer = nes.unwrap_ref().get_audio_buffer();

    env.set_float_array_region(&float_arr, 0, &buffer)
        .expect("Failed to load audio buffer");

    buffer.len() as u32
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
    palette_arr: JByteArray<'static>,
) {
    let buffer = nes.unwrap_ref().get_frame_buffer();

    let mut elements = unsafe {
        env.get_array_elements(&int_arr, jni::objects::ReleaseMode::CopyBack)
            .expect("Failed to get frame buffer")
    };

    // FIXME: Find a way to directly access JByteArray elements
    let palette = env.convert_byte_array(&palette_arr).unwrap_or_default();
    let palette = match palette_arr.is_null() {
        true => ppu::COLOR_PALETTE,
        false => palette.as_slice(),
    };

    for (i, pixel) in buffer.iter().enumerate() {
        let color_index = *pixel as usize;
        let a = 255u32;
        let r = palette[color_index] as u32;
        let g = palette[color_index + 1] as u32;
        let b = palette[color_index + 2] as u32;

        elements[i] = (a << 24 | r << 16 | g << 8 | b) as i32;
    }
}
