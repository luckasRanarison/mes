package dev.luckasranarison.mes.ui.emulator

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import dev.luckasranarison.mes.lib.FRAME_DURATION
import dev.luckasranarison.mes.ui.gamepad.GamePadLayout
import kotlinx.coroutines.delay

@Composable
fun Emulator(viewModel: EmulatorViewModel) {
    val ctx = LocalContext.current
    val emulatorView = remember { EmulatorView(ctx) }

    LaunchedEffect(Unit) {
        var lastTimestamp = System.nanoTime()

        viewModel.startEmulation()

        while (true) {
            val timestamp = System.nanoTime()
            val delta = timestamp - lastTimestamp

            if (delta >= FRAME_DURATION) {
                lastTimestamp += FRAME_DURATION
                viewModel.doFrame(emulatorView)
            } else {
                delay((FRAME_DURATION - delta) / 1_000_000)
            }
        }
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.Black)
    ) {
        AndroidView(
            factory = { emulatorView },
            modifier = Modifier
                .align(Alignment.Center)
                .fillMaxSize()
        )
        GamePadLayout(onPress = viewModel::updateController)
    }
}