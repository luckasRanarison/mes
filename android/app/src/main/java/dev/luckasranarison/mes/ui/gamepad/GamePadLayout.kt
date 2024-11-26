package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.ui.settings.FloatingSettings
import dev.luckasranarison.mes.vm.EmulatorViewModel

@Composable
fun GamePadLayout(viewModel: EmulatorViewModel) {
    var showSettings by remember { mutableStateOf(false) }

    if (showSettings) {
        FloatingSettings(
            viewModel = viewModel,
            onExit = { showSettings = false }
        )
    }

    LaunchedEffect(showSettings) {
        if (showSettings) {
            viewModel.pauseEmulation()
        } else {
            viewModel.startEmulation()
        }
    }

    Box(modifier = Modifier.fillMaxSize()) {
        DirectionPad(
            modifier = Modifier
                .align(Alignment.CenterStart)
                .padding(start = 72.dp, top = 72.dp),
            onPress = viewModel::updateController
        )
        MenuPad(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .padding(bottom = 16.dp),
            onPress = viewModel::updateController
        )
        ActionPad(
            modifier = Modifier
                .align(Alignment.CenterEnd)
                .padding(end = 48.dp, top = 72.dp),
            onPress = viewModel::updateController
        )
        IconButton(
            onClick = { showSettings = true },
            modifier = Modifier
                .align(Alignment.TopEnd)
                .padding(24.dp)
        ) {
            Icon(
                imageVector = Icons.Default.Settings,
                contentDescription = "Settings",
                tint = Color.Gray,
                modifier = Modifier.size(32.dp)
            )
        }
    }
}