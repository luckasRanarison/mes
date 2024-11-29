package dev.luckasranarison.mes.vm

import android.content.Context
import android.content.Intent
import android.media.AudioTrack
import android.net.Uri
import android.provider.DocumentsContract
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.documentfile.provider.DocumentFile
import androidx.lifecycle.*
import androidx.lifecycle.viewmodel.CreationExtras
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.data.SettingsRepository
import androidx.lifecycle.ViewModelProvider.AndroidViewModelFactory.Companion.APPLICATION_KEY
import dev.luckasranarison.mes.data.dataStore
import dev.luckasranarison.mes.lib.*
import dev.luckasranarison.mes.ui.emulator.EmulatorView
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.mapNotNull
import java.io.IOException

class EmulatorViewModel(private val settings: SettingsRepository) : ViewModel() {
    private val _romLoadingState = mutableStateOf<RomLoadingState>(RomLoadingState.None)
    private val _isRunning = mutableStateOf(false)
    private val _romFiles = mutableStateOf<List<RomFile>?>(null)
    private val _isShortcutLaunch = mutableStateOf(false)
    private val nes: NesObject = NesObject()
    private val controller = mutableStateOf(Controller())

    val romDirectory = settings.getRomDirectory().asLiveData()
    val enableApu = settings.getApuState().asLiveData()
    val colorPalette = settings.getColorPalette().asLiveData()
    val romLoadingState: State<RomLoadingState> = _romLoadingState
    val romFiles: State<List<RomFile>?> = _romFiles
    val isRunning: State<Boolean> = _isRunning
    val isShortcutLaunch: State<Boolean> = _isShortcutLaunch

    init {
        viewModelScope.launch {
            settings.getColorPalette()
                .mapNotNull { it }
                .collect{ nes.setColorPalette(it) }
        }
    }

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

    suspend fun loadRomFromDirectory(ctx: Context, uri: Uri) {
        withContext(Dispatchers.IO) {
            val parentId = DocumentsContract.getTreeDocumentId(uri)
            val childrenUri = DocumentsContract.buildChildDocumentsUriUsingTree(uri, parentId)
            val tree = DocumentFile.fromTreeUri(ctx, childrenUri)
            val files = tree?.listFiles()
                ?.mapNotNull { file -> runCatching { readRomMetadata(ctx, file) }.getOrNull() }

            _romFiles.value = files
        }
    }

    private fun readRomMetadata(ctx: Context, file: DocumentFile): RomFile {
        val stream = ctx.contentResolver.openInputStream(file.uri)

        stream?.use { handle ->
            val headerBuffer = ByteArray(4)
            val bytesRead = handle.read(headerBuffer, 0, 4)

            if (bytesRead == 4 && headerBuffer contentEquals INES_ASCII) {
                val remaining = handle.readBytes()
                val stringMetaData = Nes.serializeRomHeader(headerBuffer + remaining)
                return RomFile(file, stringMetaData)
            }
        }

        throw Exception("Not a valid iNES file")
    }

    fun setShortcutLaunch() {
        _isShortcutLaunch.value = true
    }

    fun setRomDirectory(ctx: Context, uri: Uri) {
        ctx.contentResolver.takePersistableUriPermission(
            uri,
            Intent.FLAG_GRANT_READ_URI_PERMISSION
        )
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

    fun unsetColorPalette() {
        viewModelScope.launch {
            nes.setColorPalette(null)
            settings.setColorPalette(null)
        }
    }

    fun setColorPalette(ctx: Context, uri: Uri) {
        val stream = ctx.contentResolver.openInputStream(uri)

        stream?.use { handle ->
            val palette = handle.readBytes()
            nes.setColorPalette(palette)
            viewModelScope.launch { settings.setColorPalette(palette) }
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