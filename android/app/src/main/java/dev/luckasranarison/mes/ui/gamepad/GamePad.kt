package dev.luckasranarison.mes.ui.gamepad

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.Routes
import dev.luckasranarison.mes.lib.Button

@Composable
fun GamePadLayout(onPress: (Button, Boolean) -> Unit, navController: NavHostController) {
    Box(modifier = Modifier.fillMaxSize()) {
        DirectionPad(
            modifier = Modifier
                .align(Alignment.CenterStart)
                .padding(start = 72.dp, top = 72.dp),
            onPress = onPress
        )
        MenuPad(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .padding(bottom = 16.dp),
            onPress = onPress
        )
        ActionPad(
            modifier = Modifier
                .align(Alignment.CenterEnd)
                .padding(end = 48.dp, top = 72.dp),
            onPress = onPress
        )
        IconButton(
            onClick = { navController.navigate(Routes.SETTINGS ) },
            modifier = Modifier
                .align(Alignment.TopEnd)
                .padding(24.dp)
        ) {
            Icon(
                imageVector = Icons.Default.Settings,
                contentDescription = "Settings",
                tint = Color.Gray,
                modifier = Modifier.size(32.dp)
            )
        }
    }
}