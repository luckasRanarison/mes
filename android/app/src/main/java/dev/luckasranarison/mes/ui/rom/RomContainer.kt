package dev.luckasranarison.mes.ui.rom

import android.net.Uri
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.ui.rom.sheet.BottomSheet
import dev.luckasranarison.mes.ui.theme.Typography

@Composable
fun RomContainer(rom: RomFile, onSelect: (Uri) -> Unit) {
    var isSheetVisible by remember { mutableStateOf(false) }

    if (isSheetVisible) {
        BottomSheet(
            rom = rom,
            onClose = { isSheetVisible = false },
        )
    }

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp)
            .clip(RoundedCornerShape(16.dp))
            .background(MaterialTheme.colorScheme.surface)
            .clickable { onSelect(rom.uri) }
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 8.dp, vertical = 12.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            InitialBox(
                name = rom.baseName(),
                modifier = Modifier.padding(start = 8.dp),
                foreground = MaterialTheme.colorScheme.onSecondary,
                background = MaterialTheme.colorScheme.secondary
            )

            Text(
                text = rom.baseName(),
                style = Typography.titleMedium,
                modifier = Modifier
                    .weight(1f)
                    .padding(start = 16.dp),
                maxLines = 1,
                overflow = TextOverflow.Ellipsis
            )

            IconButton(onClick = { isSheetVisible = true }) {
                Icon(
                    imageVector = Icons.Default.MoreVert,
                    contentDescription = "Details",
                    tint = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f),
                )
            }
        }
    }
}