package dev.luckasranarison.mes.ui.emulator

import android.app.Activity
import androidx.activity.compose.BackHandler
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController

@Composable
fun EmulatorBackHandler(
    controller: NavHostController,
    pauseEmulation: () -> Unit,
    resumeEmulation: () -> Unit,
    isShortcutLaunch: Boolean,
) {
    val ctx = LocalContext.current as Activity
    var showExitDialog by remember { mutableStateOf(false) }

    BackHandler { showExitDialog = true }

    LaunchedEffect(showExitDialog) {
        if (showExitDialog) {
            pauseEmulation()
        } else {
            resumeEmulation()
        }
    }

    if (showExitDialog) {
        AlertDialog(
            onDismissRequest = { showExitDialog = false },
            title = { Text(text = "Confirm to exit") },
            text = { Text(text = "Are you sure to stop the emulation?") },
            confirmButton = {
                TextButton(onClick = {
                    when (isShortcutLaunch) {
                        true -> ctx.finishAffinity()
                        else -> controller.popBackStack()
                    }
                }) {
                    Text(text = "Confirm")
                }
            },
            dismissButton = {
                TextButton(onClick = { showExitDialog = false }) {
                    Text(text = "Cancel")
                }
            }
        )
    }
}