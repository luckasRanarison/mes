package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput

@Composable
fun BaseButton(
    modifier: Modifier,
    onPress: (Boolean) -> Unit,
    content: @Composable (BoxScope.() -> Unit) = {}
) {
    Box(
        contentAlignment = Alignment.Center,
        modifier = modifier
            .pointerInput(Unit) {
                detectTapGestures(onPress = {
                    try {
                        onPress(true)
                        awaitRelease()
                    } finally {
                        onPress(false)
                    }
                })
            }
    ) { content() }
}