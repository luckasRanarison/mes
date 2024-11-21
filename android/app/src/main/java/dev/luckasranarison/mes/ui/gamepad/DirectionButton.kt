package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowLeft
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowRight
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.lib.Button

@Composable
fun DirectionButton(
    modifier: Modifier,
    icon: ImageVector,
    desc: String,
    onPress: (Boolean) -> Unit
) {
    BaseButton(
        modifier = modifier
            .background(Color.Gray.copy(alpha = 0.8f))
            .size(52.dp),
        onPress = { state -> onPress(state) },
    ) {
        Icon(
            imageVector = icon,
            contentDescription = desc,
            tint = Color.White,
            modifier = Modifier.graphicsLayer(scaleX = 1.2f, scaleY = 1.2f)
        )
    }
}

@Composable
fun DirectionPad(modifier: Modifier, onPress: (Button, Boolean) -> Unit) {
    Box(modifier = modifier.size((52 * 3).dp)) {
        DirectionButton(
            icon = Icons.Default.KeyboardArrowUp,
            desc = "Up",
            modifier = Modifier
                .align(Alignment.TopCenter)
                .clip(RoundedCornerShape(topStart = 5.dp, topEnd = 5.dp)),
            onPress = { state -> onPress(Button.Up, state) },
        )
        DirectionButton(
            icon = Icons.Default.KeyboardArrowDown,
            desc = "Down",
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .clip(RoundedCornerShape(bottomStart = 5.dp, bottomEnd = 5.dp)),
            onPress = { state -> onPress(Button.Down, state) },
        )
        DirectionButton(
            icon = Icons.AutoMirrored.Filled.KeyboardArrowLeft,
            desc = "Left",
            modifier = Modifier
                .align(Alignment.CenterStart)
                .clip(RoundedCornerShape(topStart = 5.dp, bottomStart = 5.dp)),
            onPress = { state -> onPress(Button.Left, state) },
        )
        DirectionButton(
            icon = Icons.AutoMirrored.Filled.KeyboardArrowRight,
            desc = "Right",
            modifier = Modifier
                .align(Alignment.CenterEnd)
                .clip(RoundedCornerShape(topEnd = 5.dp, bottomEnd = 5.dp)),
            onPress = { state -> onPress(Button.Right, state) },
        )
    }
}
