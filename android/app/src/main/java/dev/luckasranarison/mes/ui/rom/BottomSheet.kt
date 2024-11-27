package dev.luckasranarison.mes.ui.rom

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.data.RomHeader
import dev.luckasranarison.mes.lib.CHR_ROM_PAGE_SIZE
import dev.luckasranarison.mes.lib.PRG_RAM_SIZE
import dev.luckasranarison.mes.lib.PRG_ROM_PAGE_SIZE
import dev.luckasranarison.mes.ui.theme.Typography
import dev.luckasranarison.mes.R
import dev.luckasranarison.mes.extra.createShortcut

@Composable
@OptIn(ExperimentalMaterial3Api::class)
fun BottomSheet(
    rom: RomFile,
    onClose: () -> Unit,
) {
    val sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)
    val scope = rememberCoroutineScope()

    ModalBottomSheet(
        onDismissRequest = { onClose() },
        sheetState = sheetState,
        containerColor = MaterialTheme.colorScheme.surface,
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
        ) {
            TopRow(rom = rom)

            Spacer(modifier = Modifier.height(16.dp))

            RomAttributes(attributes = rom.getAttributes())

            MetadataList(size = rom.size, header = rom.header)

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    scope.launch { sheetState.hide(); onClose() }
                },
                modifier = Modifier.fillMaxWidth(),
                colors = ButtonDefaults.buttonColors(MaterialTheme.colorScheme.primary)
            ) {
                Text(
                    "Close",
                    style = Typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onPrimary
                )
            }
        }
    }
}

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
            )
        }
    }
}

@Composable
fun RomAttributes(attributes: List<String>) {
    if (attributes.isEmpty()) return

    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(
            text = "Attributes",
            style = Typography.bodyMedium,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.weight(1f),
        )
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            attributes.forEach {
                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(8.dp))
                        .background(MaterialTheme.colorScheme.secondary.copy(alpha = 0.2f))
                ) {
                    Text(
                        text = it,
                        style = MaterialTheme.typography.titleSmall,
                        modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
                    )
                }
            }
        }
    }
}

@Composable
fun Metadata(key: String, value: String) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(
            text = key,
            style = Typography.bodyMedium,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.weight(1f),
        )
        Text(
            text = value,
            style = Typography.bodyMedium,
            modifier = Modifier.weight(1f),
            textAlign = TextAlign.End,
            color = MaterialTheme.colorScheme.onSurface
        )
    }
}

@Composable
fun MetadataList(size: Long, header: RomHeader) {
    Metadata("Size", "${size / 1024} KB")
    Metadata("Mapper", header.mapper.toString())
    Metadata("Mirroring", header.mirroring)
    Metadata("Battery", if (header.battery) "Yes" else "No")
    Metadata("PRG ROM", formatPage(header.prgRomPages, PRG_ROM_PAGE_SIZE))
    Metadata("PRG RAM", formatPage(header.prgRamPages, PRG_RAM_SIZE))
    Metadata("CHR ROM", formatPage(header.chrRomPages, CHR_ROM_PAGE_SIZE))
}

fun formatPage(count: Byte, size: Int) =
    if (count > 0) "$count (${count * size / 1024} KB)" else "None"