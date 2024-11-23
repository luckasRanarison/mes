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
import dev.luckasranarison.mes.lib.createAudioTrack
import dev.luckasranarison.mes.ui.gamepad.GamePadLayout

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
        GamePadLayout(navController = controller, onPress = viewModel::updateController)
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
fun EmulatorBackHandler(
    controller: NavHostController,
    pauseEmulation: () -> Unit,
    resumeEmulation: () -> Unit
) {
    var showExitDialog by remember { mutableStateOf(false) }

    BackHandler {
        showExitDialog = true
        pauseEmulation()
    }

    if (showExitDialog) {
        AlertDialog(
            onDismissRequest = {
                showExitDialog = false
                resumeEmulation()
            },
            title = { Text(text = "Confirm to exit") },
            text = { Text(text = "Are you sure to stop the emulation?") },
            confirmButton = {
                TextButton(onClick = { controller.popBackStack() }) {
                    Text(text = "Confirm")
                }
            },
            dismissButton = {
                TextButton(onClick = {
                    showExitDialog = false
                    resumeEmulation()
                }) {
                    Text(text = "Cancel")
                }
            }
        )
    }
}