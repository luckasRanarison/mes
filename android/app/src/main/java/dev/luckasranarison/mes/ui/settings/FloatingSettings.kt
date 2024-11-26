package dev.luckasranarison.mes.ui.settings

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import dev.luckasranarison.mes.vm.EmulatorViewModel

@Composable
fun FloatingSettings(viewModel: EmulatorViewModel, onExit: () -> Unit) {
    Dialog(
        onDismissRequest = onExit,
        properties = DialogProperties(
            usePlatformDefaultWidth = false,
        )
    ) {
        Box(
            modifier = Modifier
                .fillMaxWidth(0.6f)
                .fillMaxHeight()
        ) {
            Card(
                modifier = Modifier
                    .clip(RoundedCornerShape(24.dp))
                    .align(Alignment.Center)
            ) {
                Settings(viewModel = viewModel, onExit = onExit)
            }
        }
    }
}