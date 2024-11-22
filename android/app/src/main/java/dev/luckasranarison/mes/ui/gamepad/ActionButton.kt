package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.luckasranarison.mes.lib.Button

@Composable
fun ActionButton(text: String, onPress: (Boolean) -> Unit) {
    BaseButton(
        modifier = Modifier
            .clip(CircleShape)
            .size(48.dp),
        onPress = { state -> onPress(state) },
    ) {
        Text(
            text = text,
            fontSize = 20.sp,
            fontWeight = FontWeight.Bold,
            color = Color.White
        )
    }
}

@Composable
fun ActionPad(modifier: Modifier, onPress: (Button, Boolean) -> Unit) {
    Row(modifier = modifier, horizontalArrangement = Arrangement.spacedBy(24.dp)) {
        ActionButton(text = "B") { state -> onPress(Button.B, state) }
        ActionButton(text = "A") { state -> onPress(Button.A, state) }
    }
}