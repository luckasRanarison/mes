package dev.luckasranarison.mes.lib

import android.util.Log

typealias NesPtr = Long

object Nes {
    external fun init(): NesPtr
    external fun reset(nes: NesPtr)
    external fun setCartridge(nes: NesPtr, bytes: ByteArray)
    external fun stepFrame(nes: NesPtr)
    external fun stepVBlank(nes: NesPtr)
    external fun fillAudioBuffer(nes: NesPtr, buffer: FloatArray)
    external fun clearAudioBuffer(nes: NesPtr)
    external fun fillFrameBuffer(nes: NesPtr, buffer: IntArray)
    external fun setControllerState(nes: NesPtr, id: Long, state: Byte)
    external fun free(nes: NesPtr)
}

const val AUDIO_BUFFER_SIZE = 1024
const val SCREEN_WIDTH = 256
const val SCREEN_HEIGHT = 240
const val FRAME_BUFFER_SIZE = SCREEN_WIDTH * SCREEN_HEIGHT
const val FRAME_DURATION = 1_000_000_000 / 60

class NesObject {
    private val ptr = Nes.init()
    private val audioBuffer = FloatArray(AUDIO_BUFFER_SIZE)
    private val frameBuffer = IntArray(FRAME_BUFFER_SIZE)

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
        Nes.fillFrameBuffer(ptr, frameBuffer)
        return frameBuffer
    }

    fun updateAudioBuffer(): FloatArray {
        Nes.fillAudioBuffer(ptr, audioBuffer)
        return audioBuffer
    }

    fun free() {
        Nes.free(ptr)
        Log.i("mes", "Emulator instance was destroyed")
    }

}