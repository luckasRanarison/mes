package dev.luckasranarison.mes.ui.rom.sheet

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.R
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.extra.createShortcut
import dev.luckasranarison.mes.ui.rom.InitialBox
import dev.luckasranarison.mes.ui.theme.Typography

@Composable
fun TopRow(rom: RomFile) {
    val ctx = LocalContext.current

    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = Modifier.fillMaxWidth()
    ) {
        InitialBox(
            name = rom.baseName(),
            modifier = Modifier,
            foreground = MaterialTheme.colorScheme.onPrimary,
            background = MaterialTheme.colorScheme.primary
        )

        Spacer(modifier = Modifier.width(16.dp))

        Text(
            text = rom.baseName(),
            style = Typography.bodyLarge.copy(fontWeight = FontWeight.Bold),
            modifier = Modifier.weight(1f),
            maxLines = 1,
            overflow = TextOverflow.Ellipsis
        )

        IconButton(onClick = { createShortcut(ctx, rom) }) {
            Icon(
                painter = painterResource(id = R.drawable.app_shortcut),
                contentDescription = "Shortcut",
                modifier = Modifier.size(20.dp)
            )
        }
    }
}