package dev.luckasranarison.mes.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

private val DarkColorScheme = darkColorScheme(
    primary = ColorScheme.Primary,
    secondary = ColorScheme.Secondary,
    tertiary = ColorScheme.Smoke,
    background = ColorScheme.Dark,
    onPrimary = ColorScheme.Light,
    onSecondary = ColorScheme.Light,
    surface = ColorScheme.Dark,
    surfaceTint = ColorScheme.Dark,
)

private val LightColorScheme = lightColorScheme(
    primary = ColorScheme.Primary,
    secondary = ColorScheme.Secondary,
    tertiary = ColorScheme.Smoke,
    background = ColorScheme.Light,
    onPrimary = ColorScheme.Light,
    onSecondary = ColorScheme.Light,
    surface = ColorScheme.Light,
    surfaceTint = ColorScheme.Light,
)

@Composable
fun MesTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    val colorScheme = when {
        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content
    )
}