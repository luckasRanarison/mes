package dev.luckasranarison.mes.ui.rom

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.ui.theme.Typography

@Composable
fun InitialBox(
    name: String,
    modifier: Modifier,
    foreground: Color,
    background: Color
) {
    Box(
        modifier = modifier
            .size(40.dp)
            .clip(RoundedCornerShape(8.dp))
            .background(background),
        contentAlignment = Alignment.Center
    ) {
        Text(
            text = name
                .split(" ")
                .take(3).mapNotNull { s -> s.first().takeIf { it.isLetter() } }
                .joinToString("")
                .ifEmpty { "NES" },
            style = Typography.titleSmall,
            color = foreground,
        )
    }
}
