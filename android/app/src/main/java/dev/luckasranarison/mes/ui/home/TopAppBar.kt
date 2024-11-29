package dev.luckasranarison.mes.ui.home

import androidx.compose.foundation.layout.Row
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.Routes

@Composable
@OptIn(ExperimentalMaterial3Api::class)
fun HomeTopAppBar(controller: NavHostController) {
    TopAppBar(
        title = { Text("Mes Emulator") },
        actions = {
            Row {
                IconButton(onClick = { controller.navigate(Routes.INFO) }) {
                    Icon(
                        imageVector = Icons.Default.Info,
                        contentDescription = "Info"
                    )
                }
                IconButton(onClick = { controller.navigate(Routes.SETTINGS) }) {
                    Icon(
                        imageVector = Icons.Default.Settings,
                        contentDescription = "Settings"
                    )
                }
            }
        }
    )
}