package dev.luckasranarison.mes.ui.emulator

import androidx.activity.compose.BackHandler
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.navigation.NavHostController

@Composable
fun EmulatorBackHandler(
    controller: NavHostController,
    pauseEmulation: () -> Unit,
    resumeEmulation: () -> Unit
) {
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
                TextButton(onClick = { controller.popBackStack() }) {
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