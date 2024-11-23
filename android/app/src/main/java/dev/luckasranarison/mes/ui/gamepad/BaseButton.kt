package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput

@Composable
fun BaseButton(
    modifier: Modifier,
    onPress: (Boolean) -> Unit,
    content: @Composable (BoxScope.() -> Unit) = {}
) {
    var isPressed by remember { mutableStateOf(false) }

    Box(
        contentAlignment = Alignment.Center,
        modifier = modifier
            .background(
                Color.Gray.copy(
                    alpha = if (isPressed) 0.5f else 0.8f
                )
            )
            .pointerInput(Unit) {
                detectTapGestures(onPress = {
                    try {
                        onPress(true)
                        isPressed = true
                        awaitRelease()
                    } finally {
                        onPress(false)
                        isPressed = false
                    }
                })
            }
    ) { content() }
}