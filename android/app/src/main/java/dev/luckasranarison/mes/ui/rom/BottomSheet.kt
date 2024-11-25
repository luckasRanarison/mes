package dev.luckasranarison.mes.ui.rom

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.data.RomHeader
import dev.luckasranarison.mes.lib.CHR_ROM_PAGE_SIZE
import dev.luckasranarison.mes.lib.PRG_RAM_SIZE
import dev.luckasranarison.mes.lib.PRG_ROM_PAGE_SIZE
import dev.luckasranarison.mes.ui.theme.Typography
import kotlinx.coroutines.launch

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
                .padding(16.dp)
        ) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.fillMaxWidth()
            ) {
                InitialBox(
                    name = rom.name,
                    modifier = Modifier,
                    foreground = MaterialTheme.colorScheme.onPrimary,
                    background = MaterialTheme.colorScheme.primary
                )

                Spacer(modifier = Modifier.width(16.dp))

                Text(
                    text = rom.name,
                    style = Typography.bodyLarge.copy(fontWeight = FontWeight.Bold),
                    modifier = Modifier.weight(1f),
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }

            Spacer(modifier = Modifier.height(16.dp))

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
            modifier = Modifier.weight(1f),
            color = MaterialTheme.colorScheme.primary
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
    Metadata("PRG ROM", ("${header.prgRomPages} (${header.prgRomPages * PRG_ROM_PAGE_SIZE / 1024} KB)"))
    Metadata("PRG RAM", ("${header.prgRamPages} (${header.prgRamPages * PRG_RAM_SIZE / 1024} KB)"))
    Metadata("CHR ROM", ("${header.chrRomPages} (${header.chrRomPages * CHR_ROM_PAGE_SIZE / 1024} KB)"))
}