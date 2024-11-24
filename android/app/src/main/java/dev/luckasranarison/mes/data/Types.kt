package dev.luckasranarison.mes.data

import android.net.Uri
import androidx.documentfile.provider.DocumentFile
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class RomHeader(
    @SerialName("prg_rom_pages") val prgRomPages: Byte,
    @SerialName("chr_rom_pages") val chrRomPages: Byte,
    @SerialName("prg_ram_pages") val prgRamPages: Byte,
    val mirroring: String,
    val battery: Boolean,
    val trainer: Boolean,
    val mapper: Short
)

data class RomFile(
    val name: String,
    val uri: Uri,
    val size: Long,
    val header: RomHeader
) {
    constructor(file: DocumentFile, metadata: String) : this(
        name = file.name ?: "Unknown",
        uri = file.uri,
        size = file.length(),
        header = Json.decodeFromString<RomHeader>(metadata)
    )
}