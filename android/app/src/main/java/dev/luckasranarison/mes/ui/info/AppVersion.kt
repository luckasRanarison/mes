package dev.luckasranarison.mes.ui.info

import android.content.Context
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.unit.dp

@Composable
fun AppVersion() {
    val ctx = LocalContext.current
    val clipboardManager = LocalClipboardManager.current
    val info = remember { getAppVersion(ctx) }

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clickable {
                clipboardManager.setText(
                    AnnotatedString("Mes v${info.versionName}")
                )
            }
            .padding(horizontal = 16.dp, vertical = 16.dp)
    ) {
        Text(text = "Version")
        Text(
            text = info.versionName,
            color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.6f)
        )
    }
}

fun getAppVersion(ctx: Context) = ctx.packageManager?.getPackageInfo(ctx.packageName, 0)
    ?: throw Exception("Failed to get package info")