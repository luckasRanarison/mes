package dev.luckasranarison.mes.extra

import android.content.Context
import android.content.Intent
import android.content.pm.ShortcutInfo
import android.content.pm.ShortcutManager
import androidx.core.content.ContextCompat.getSystemService
import dev.luckasranarison.mes.MainActivity
import dev.luckasranarison.mes.data.RomFile

fun createNewShortcut(ctx: Context, rom: RomFile) {
    val shortcutManager = getSystemService(ctx, ShortcutManager::class.java)

    val intent = Intent(Intent.ACTION_VIEW, rom.uri, ctx, MainActivity::class.java)
    intent.putExtra("path", rom.uri.toString())

    val shortcut = ShortcutInfo.Builder(ctx, rom.uri.toString())
        .setShortLabel(rom.baseName())
        .setIntent(intent)
        .build()

    shortcutManager?.requestPinShortcut(shortcut, null)
}