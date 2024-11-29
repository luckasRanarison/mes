package dev.luckasranarison.mes.ui.settings

import android.content.Context
import androidx.core.net.toUri
import androidx.documentfile.provider.DocumentFile

fun String.toPathName(context: Context): String {
    val uri = this.toUri()
    val documentFile = DocumentFile.fromTreeUri(context, uri)
    return documentFile?.name ?: "Unknown Directory"
}