package dev.luckasranarison.mes.ui.system

import android.app.Activity
import android.content.pm.ActivityInfo
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.navigation.NavController
import androidx.navigation.NavHostController

@Composable
fun ScreenEffect(navController: NavHostController) {
    val view = LocalView.current
    val ctx = LocalContext.current as Activity

    DisposableEffect(Unit) {
        val insetsController = WindowCompat.getInsetsController(ctx.window, view)
        val systemBars = WindowInsetsCompat.Type.systemBars()

        insetsController.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

        val listener = NavController.OnDestinationChangedListener { _, dest, _ ->
            if (dest.route == "emulator" && !view.isInEditMode) {
                insetsController.hide(systemBars)
                ctx.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_USER_LANDSCAPE
            } else {
                insetsController.show(systemBars)
                ctx.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_USER
            }
        }

        navController.addOnDestinationChangedListener(listener)

        onDispose {
            navController.removeOnDestinationChangedListener(listener)
        }
    }
}

