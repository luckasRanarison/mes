package dev.luckasranarison.mes.lib

import android.util.Log

typealias NesPtr = Long

object Nes {
    external fun init(): NesPtr
    external fun reset(nes: NesPtr)
    external fun setCartridge(nes: NesPtr, bytes: ByteArray)
    external fun stepFrame(nes: NesPtr)
    external fun stepVBlank(nes: NesPtr)
    external fun fillAudioBuffer(nes: NesPtr, buffer: FloatArray): Int
    external fun clearAudioBuffer(nes: NesPtr)
    external fun fillFrameBuffer(nes: NesPtr, buffer: IntArray, palette: ByteArray?)
    external fun setControllerState(nes: NesPtr, id: Long, state: Byte)
    external fun free(nes: NesPtr)
    external fun serializeRomHeader(rom: ByteArray): String
}

const val AUDIO_BUFFER_SIZE = 1024
const val SCREEN_WIDTH = 256
const val SCREEN_HEIGHT = 240
const val FRAME_BUFFER_SIZE = SCREEN_WIDTH * SCREEN_HEIGHT
const val COLOR_PALETTE_SIZE = 192
const val FRAME_DURATION = 1_000_000_000 / 60
const val PRG_ROM_PAGE_SIZE = 16384;
const val PRG_RAM_SIZE = 8192;
const val CHR_ROM_PAGE_SIZE = 8192;
val INES_ASCII = byteArrayOf(0x4E, 0x45, 0x53, 0x1A)

class NesObject {
    private val ptr = Nes.init()
    private val audioBuffer = FloatArray(AUDIO_BUFFER_SIZE)
    private val frameBuffer = IntArray(FRAME_BUFFER_SIZE)
    private var colorPalette: ByteArray? = null

    init {
        Log.i("mes", "Emulator instance was created")
    }

    fun reset() = Nes.reset(ptr)
    fun setCartridge(bytes: ByteArray) = Nes.setCartridge(ptr, bytes)
    fun stepFrame() = Nes.stepFrame(ptr)
    fun stepVBlank() = Nes.stepVBlank(ptr)
    fun clearAudioBuffer() = Nes.clearAudioBuffer(ptr)
    fun setControllerState(id: Long, state: Byte) = Nes.setControllerState(ptr, id, state)

    fun updateFrameBuffer(): IntArray {
        Nes.fillFrameBuffer(ptr, frameBuffer, colorPalette)
        return frameBuffer
    }

    fun updateAudioBuffer(): Pair<FloatArray, Int> {
        val length = Nes.fillAudioBuffer(ptr, audioBuffer)
        return Pair(audioBuffer, length)
    }

    fun setColorPalette(palette: ByteArray?) {
        if (palette == null || palette.size == COLOR_PALETTE_SIZE) {
            colorPalette = palette
        } else {
            throw Exception("Invalid color palette")
        }
    }

    fun free() {
        Nes.free(ptr)
        Log.i("mes", "Emulator instance was destroyed")
    }
}