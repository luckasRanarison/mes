package dev.luckasranarison.mes.ui.info

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController

@Composable
fun Info(controller: NavHostController) {
    Scaffold(
        topBar = {
            InfoTopAppBar(onExit = { controller.popBackStack() })
        },
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxSize()
        ) {
            AppIcon()

            HorizontalDivider(thickness = 1.dp)

            AppVersion()
        }
    }
}