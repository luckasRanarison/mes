package dev.luckasranarison.mes.ui.rom.sheet

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.data.RomFile
import dev.luckasranarison.mes.lib.CHR_ROM_PAGE_SIZE
import dev.luckasranarison.mes.lib.PRG_RAM_SIZE
import dev.luckasranarison.mes.lib.PRG_ROM_PAGE_SIZE
import dev.luckasranarison.mes.ui.theme.Typography

fun formatPage(count: Byte, size: Int) =
    if (count > 0) "$count (${count * size / 1024} KB)" else "None"

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
fun MetadataList(rom: RomFile) {
    val attributes = rom.getAttributes()

    Metadata("Attributes", if (attributes.isEmpty()) "None" else attributes.joinToString())
    Metadata("Size", "${rom.size / 1024} KB")
    Metadata("Mapper", rom.header.mapper.toString())
    Metadata("Mirroring", rom.header.mirroring)
    Metadata("Battery", if (rom.header.battery) "Yes" else "No")
    Metadata("PRG ROM", formatPage(rom.header.prgRomPages, PRG_ROM_PAGE_SIZE))
    Metadata("PRG RAM", formatPage(rom.header.prgRamPages, PRG_RAM_SIZE))
    Metadata("CHR ROM", formatPage(rom.header.chrRomPages, CHR_ROM_PAGE_SIZE))
}