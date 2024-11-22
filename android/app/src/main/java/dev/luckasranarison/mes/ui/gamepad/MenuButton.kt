package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
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
fun MenuButton(text: String, onPress: (Boolean) -> Unit) {
    BaseButton(
        modifier = Modifier.clip(RoundedCornerShape(5.dp)),
        onPress = { state -> onPress(state) },
    ) {
        Text(
            text = text,
            fontSize = 10.sp,
            modifier = Modifier.padding(vertical = 2.dp, horizontal = 10.dp),
            fontWeight = FontWeight.Bold,
            color = Color.White
        )
    }
}

@Composable
fun MenuPad(modifier: Modifier, onPress: (Button, Boolean) -> Unit) {
    Row(modifier = modifier, horizontalArrangement = Arrangement.spacedBy(16.dp)) {
        MenuButton(text = "SELECT") { state -> onPress(Button.Select, state) }
        MenuButton(text = "START") { state -> onPress(Button.Start, state) }
    }
}