package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.lib.Button

@Composable
fun GamePadLayout(onPress: (Button, Boolean) -> Unit) {
    Box(modifier = Modifier.fillMaxSize()) {
        DirectionPad(
            modifier = Modifier
                .align(Alignment.CenterStart)
                .padding(start = 72.dp, top = 72.dp),
            onPress = onPress
        )
        MenuPad(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .padding(bottom = 16.dp),
            onPress = onPress
        )
        ActionPad(
            modifier = Modifier
                .align(Alignment.CenterEnd)
                .padding(end = 48.dp, top = 72.dp),
            onPress = onPress
        )
    }
}