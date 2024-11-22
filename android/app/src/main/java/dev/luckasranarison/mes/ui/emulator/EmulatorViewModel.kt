package dev.luckasranarison.mes.ui.emulator

import android.media.AudioTrack
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import dev.luckasranarison.mes.lib.Button
import dev.luckasranarison.mes.lib.Controller
import dev.luckasranarison.mes.lib.NesObject

class EmulatorViewModel : ViewModel() {
    private val _romLoadingState = mutableStateOf<RomLoadingState>(RomLoadingState.None)
    private val _isRunning = mutableStateOf(false)
    private val nes: NesObject = NesObject()
    private val controller = mutableStateOf(Controller())

    val romLoadingState: State<RomLoadingState> = _romLoadingState
    val isRunning: State<Boolean> = _isRunning

    override fun onCleared() {
        super.onCleared()
        nes.free()
    }

    fun setRom(rom: ByteArray) {
        nes.setCartridge(rom)
        nes.reset()
    }

    fun setLoadStatus(state: RomLoadingState) {
        _romLoadingState.value = state
    }

    fun pauseEmulation() {
        _isRunning.value = false
    }

    fun startEmulation() {
        _isRunning.value = true
    }

    fun updateController(button: Button, state: Boolean) {
        controller.value = controller.value.update(button, state)
    }

    fun doFrame(view: EmulatorView, audio: AudioTrack) {
        nes.stepFrame()

        val frameBuffer = nes.updateFrameBuffer()
        view.updateScreenData(frameBuffer)

        val (audioBuffer, length) = nes.updateAudioBuffer()
        audio.write(audioBuffer, 0, length, AudioTrack.WRITE_NON_BLOCKING)
        nes.clearAudioBuffer()

        nes.setControllerState(0, controller.value.state())
        nes.stepVBlank()
    }
}

sealed class RomLoadingState {
    data object None : RomLoadingState()
    data object Success : RomLoadingState()
    data class Error(val message: String) : RomLoadingState()
}