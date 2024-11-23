package dev.luckasranarison.mes.ui.emulator

import android.content.Context
import android.content.Intent
import android.media.AudioTrack
import android.net.Uri
import android.provider.DocumentsContract
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.documentfile.provider.DocumentFile
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewmodel.CreationExtras
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.data.SettingsRepository
import dev.luckasranarison.mes.lib.Button
import dev.luckasranarison.mes.lib.Controller
import dev.luckasranarison.mes.lib.NesObject
import androidx.lifecycle.ViewModelProvider.AndroidViewModelFactory.Companion.APPLICATION_KEY
import androidx.lifecycle.asLiveData
import androidx.lifecycle.viewModelScope
import dev.luckasranarison.mes.data.dataStore
import dev.luckasranarison.mes.lib.FRAME_DURATION
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import java.io.IOException

class EmulatorViewModel(private val settings: SettingsRepository) : ViewModel() {
    private val _romLoadingState = mutableStateOf<RomLoadingState>(RomLoadingState.None)
    private val _isRunning = mutableStateOf(false)
    private val _romFiles = mutableStateOf<List<RomFile>?>(null)
    private val nes: NesObject = NesObject()
    private val controller = mutableStateOf(Controller())

    val romDirectory = settings.getRomDirectory().asLiveData()
    val enableApu = settings.getApuState().asLiveData()
    val romLoadingState: State<RomLoadingState> = _romLoadingState
    val romFiles: State<List<RomFile>?> = _romFiles
    val isRunning: State<Boolean> = _isRunning

    override fun onCleared() {
        super.onCleared()
        nes.free()
    }

    fun loadRomFromFile(ctx: Context, uri: Uri) {
        try {
            val stream = ctx.contentResolver.openInputStream(uri)

            stream.use { handle ->
                val rom = handle?.readBytes() ?: throw IOException("Failed to read ROM")
                nes.setCartridge(rom)
                nes.reset()
                _romLoadingState.value = RomLoadingState.Success
            }
        } catch (err: Exception) {
            val message = err.message ?: "An unknown error occurred"
            _romLoadingState.value = RomLoadingState.Error(message)
        }
    }

    fun loadRomFromDirectory(ctx: Context, uri: Uri) {
        val parentId = DocumentsContract.getTreeDocumentId(uri)
        val childrenUri = DocumentsContract.buildChildDocumentsUriUsingTree(uri, parentId)
        val tree = DocumentFile.fromTreeUri(ctx, childrenUri)
        val files = tree?.listFiles()
            ?.filter { file -> file.name?.endsWith(".nes") ?: false }
            ?.map { file -> RomFile(name = file.name ?: "Unknown", uri = file.uri) }

        _romFiles.value = files
    }

    fun setRomDirectory(ctx: Context, uri: Uri) {
        ctx.contentResolver.takePersistableUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
        viewModelScope.launch { settings.setRomDirectory(uri.toString()) }
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

    fun toggleApuState() {
        viewModelScope.launch {
            settings.toggleApuState()
        }
    }

    suspend fun runMainLoop(view: EmulatorView, audio: AudioTrack) {
        var lastTimestamp = System.nanoTime()

        while (isRunning.value) {
            val timestamp = System.nanoTime()
            val delta = timestamp - lastTimestamp

            if (delta >= FRAME_DURATION) {
                lastTimestamp += FRAME_DURATION
                stepFrame(view, audio)
            } else {
                delay((FRAME_DURATION - delta) / 1_000_000)
            }
        }
    }

    private fun stepFrame(view: EmulatorView, audio: AudioTrack) {
        nes.stepFrame()

        val frameBuffer = nes.updateFrameBuffer()
        view.updateScreenData(frameBuffer)

        val (audioBuffer, length) = nes.updateAudioBuffer()

        if (enableApu.value != false) {
            audio.write(audioBuffer, 0, length, AudioTrack.WRITE_NON_BLOCKING)
            nes.clearAudioBuffer()
        }

        nes.setControllerState(0, controller.value.state())
        nes.stepVBlank()
    }

    companion object {
        val Factory: ViewModelProvider.Factory = object : ViewModelProvider.Factory {
            @Suppress("UNCHECKED_CAST")
            override fun <T : ViewModel> create(
                modelClass: Class<T>,
                extras: CreationExtras
            ): T {
                val application = extras[APPLICATION_KEY]!!
                val store = application.dataStore
                val repository = SettingsRepository(store)
                return EmulatorViewModel(repository) as T
            }
        }
    }
}

sealed class RomLoadingState {
    data object None : RomLoadingState()
    data object Success : RomLoadingState()
    data class Error(val message: String) : RomLoadingState()
}