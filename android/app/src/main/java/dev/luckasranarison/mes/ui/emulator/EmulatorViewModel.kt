package dev.luckasranarison.mes.ui.emulator

import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import dev.luckasranarison.mes.lib.Button
import dev.luckasranarison.mes.lib.Controller
import dev.luckasranarison.mes.lib.NesObject

class EmulatorViewModel : ViewModel() {
    private val _romLoadingState = mutableStateOf<RomLoadingState>(RomLoadingState.None)
    private val nes: NesObject = NesObject()
    private val controller = mutableStateOf(Controller())

    val romLoadingState: State<RomLoadingState> = _romLoadingState

    fun setRom(rom: ByteArray) {
        nes.setCartridge(rom)
        nes.reset()
    }

    fun setLoadSuccess() {
        _romLoadingState.value = RomLoadingState.Success
    }

    fun setLoadError(err: Exception) {
        _romLoadingState.value = RomLoadingState.Error(err.message ?: "An unknown error occurred")
    }

    fun startEmulation() {
        _romLoadingState.value = RomLoadingState.None
    }

    fun updateController(button: Button, state: Boolean) {
        controller.value = controller.value.update(button, state)
    }

    fun doFrame(view: EmulatorView) {
        nes.stepFrame()
        view.updateScreenData(nes.updateFrameBuffer())
        nes.setControllerState(0, controller.value.state())
        nes.stepVBlank()
    }

    override fun onCleared() {
        super.onCleared()
        nes.free()
    }
}

sealed class RomLoadingState {
    data object None : RomLoadingState()
    data object Success : RomLoadingState()
    data class Error(val message: String) : RomLoadingState()
}