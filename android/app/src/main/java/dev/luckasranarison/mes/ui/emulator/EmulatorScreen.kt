package dev.luckasranarison.mes.ui.emulator

import android.app.Activity
import android.content.pm.ActivityInfo
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.*
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.view.*
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.lib.FRAME_DURATION
import dev.luckasranarison.mes.lib.createAudioTrack
import dev.luckasranarison.mes.ui.gamepad.GamePadLayout
import kotlinx.coroutines.delay

@Composable
fun Emulator(viewModel: EmulatorViewModel, controller: NavHostController) {
    val ctx = LocalContext.current
    val emulatorView = remember { EmulatorView(ctx) }
    val audioTrack = remember { createAudioTrack() }
    val isRunning by viewModel.isRunning

    LaunchedEffect(Unit) {
        viewModel.startEmulation()
    }

    DisposableEffect(Unit) {
        audioTrack.play()

        onDispose {
            audioTrack.stop()
            audioTrack.release()
        }
    }

    LaunchedEffect(isRunning) {
        var lastTimestamp = System.nanoTime()

        while (isRunning) {
            val timestamp = System.nanoTime()
            val delta = timestamp - lastTimestamp

            if (delta >= FRAME_DURATION) {
                lastTimestamp += FRAME_DURATION
                viewModel.doFrame(emulatorView, audioTrack)
            } else {
                delay((FRAME_DURATION - delta) / 1_000_000)
            }
        }
    }

    EmulatorBackHandler(controller)

    FullScreenLandscapeBox {
        AndroidView(
            factory = { emulatorView },
            modifier = Modifier
                .align(Alignment.Center)
                .fillMaxSize()
        )
        GamePadLayout(onPress = viewModel::updateController)
    }
}

@Composable
fun FullScreenLandscapeBox(content: @Composable (BoxScope.() -> Unit)) {
    val view = LocalView.current
    val ctx = LocalContext.current as Activity

    DisposableEffect(Unit) {
        val insetsController = WindowCompat.getInsetsController(ctx.window, view)
        val systemBars = WindowInsetsCompat.Type.systemBars()

        insetsController.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

        insetsController.hide(systemBars)
        ctx.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_USER_LANDSCAPE

        onDispose {
            insetsController.show(systemBars)
            ctx.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_USER
        }
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.Black)
    ) { content() }
}

@Composable
fun EmulatorBackHandler(controller: NavHostController) {
    var showExitDialog by remember { mutableStateOf(false) }

    BackHandler {
        showExitDialog = true
    }

    if (showExitDialog) {
        AlertDialog(
            onDismissRequest = { showExitDialog = false },
            title = { Text("Confirm to exit") },
            text = { Text("Are you sure to stop the emulation?") },
            confirmButton = {
                TextButton(onClick = { controller.popBackStack() }) {
                    Text("Confirm")
                }
            },
            dismissButton = {
                TextButton(onClick = { showExitDialog = false }) {
                    Text("Cancel")
                }
            }
        )
    }
}