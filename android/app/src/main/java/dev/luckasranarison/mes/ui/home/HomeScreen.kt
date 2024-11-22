package dev.luckasranarison.mes.ui.home

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.ui.theme.Light
import dev.luckasranarison.mes.ui.theme.Primary

@Composable
@OptIn(ExperimentalMaterial3Api::class)
fun Home(pickFile: () -> Unit, controller: NavHostController) {
    Scaffold(
        topBar = { TopAppBar({ Text("Mes Emulator") }) },
    ) { innerPadding ->
        Box(
            modifier = Modifier.padding(innerPadding)
        ) {
            FileUploader(onUpload = pickFile)
        }
    }
}

@Composable
fun FileUploader(onUpload: () -> Unit) {
    Column(
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Button(
            onClick = onUpload,
            colors = ButtonDefaults.buttonColors(contentColor = Light, containerColor = Primary)
        ) {
            Text(text = "Upload")
        }
    }
}