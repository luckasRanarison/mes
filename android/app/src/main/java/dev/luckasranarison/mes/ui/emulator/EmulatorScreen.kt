package dev.luckasranarison.mes.ui.emulator

import androidx.compose.foundation.layout.*
import androidx.compose.runtime.*
import androidx.compose.ui.*
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.lib.createAudioTrack
import dev.luckasranarison.mes.ui.gamepad.GamePadLayout
import dev.luckasranarison.mes.vm.EmulatorViewModel

@Composable
fun Emulator(viewModel: EmulatorViewModel, controller: NavHostController) {
    val ctx = LocalContext.current
    val emulatorView = remember { EmulatorView(ctx) }
    val audioTrack = remember { createAudioTrack() }
    val isRunning by viewModel.isRunning

    DisposableEffect(Unit) {
        viewModel.startEmulation()
        audioTrack.play()

        onDispose {
            audioTrack.stop()
            audioTrack.release()
        }
    }

    LaunchedEffect(isRunning) {
        viewModel.runMainLoop(emulatorView, audioTrack)
    }

    EmulatorBackHandler(
        controller = controller,
        pauseEmulation = viewModel::pauseEmulation,
        resumeEmulation = viewModel::startEmulation
    )

    FullScreenLandscapeBox {
        AndroidView(
            factory = { emulatorView },
            modifier = Modifier
                .align(Alignment.Center)
                .fillMaxSize()
        )
        GamePadLayout(viewModel = viewModel)
    }
}