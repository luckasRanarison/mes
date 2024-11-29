package dev.luckasranarison.mes.ui.emulator

import android.app.Activity
import android.content.pm.ActivityInfo
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

@Composable
fun FullScreenLandscapeBox(content: @Composable (BoxScope.() -> Unit)) {
    val view = LocalView.current
    val ctx = LocalContext.current as Activity

    DisposableEffect(Unit) {
        val insetsController = WindowCompat.getInsetsController(ctx.window, view)
        val systemBars = WindowInsetsCompat.Type.systemBars()

        insetsController.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

        insetsController.hide(systemBars)
        ctx.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_USER_LANDSCAPE

        onDispose {
            insetsController.show(systemBars)
            ctx.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_USER
        }
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.Black)
    ) { content() }
}