package dev.luckasranarison.mes.data

import android.net.Uri
import androidx.documentfile.provider.DocumentFile

data class RomFile(
    val name: String,
    val uri: Uri,
    val size: Long,
) {
    constructor(file: DocumentFile) : this(
        name = file.name ?: "Unknown",
        uri = file.uri,
        size = file.length(),
    )
}