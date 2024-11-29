package dev.luckasranarison.mes.ui.info

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.Routes
import dev.luckasranarison.mes.ui.shared.GenericTopAppBar

const val AUTHOR_EMAIL = "luckasranarison@gmail.com"
const val REPOSITORY_URL = "https://github.com/luckasranarison/mes"

@Composable
fun Info(controller: NavHostController) {
    val ctx = LocalContext.current
    val clipboardManager = LocalClipboardManager.current
    val uriHandler = LocalUriHandler.current
    val version = remember { getAppVersion(ctx) }

    Scaffold(
        topBar = {
            GenericTopAppBar(
                title = "About",
                onExit = { controller.popBackStack() }
            )
        },
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxSize()
        ) {
            AppIcon()

            HorizontalDivider(thickness = 1.dp)

            Section(
                title = "Version",
                description = version,
                onClick = { clipboardManager.setText(AnnotatedString("Mes v${version}")) }
            )
            Section(
                title = "Author",
                description = AUTHOR_EMAIL,
                onClick = { uriHandler.openUri(makeMailMessage(AUTHOR_EMAIL)) }
            )
            Section(
                title = "Source",
                description = REPOSITORY_URL,
                onClick = { uriHandler.openUri(REPOSITORY_URL) }
            )
            Section(
                title = "Open source license",
                onClick = { controller.navigate(Routes.LICENSE) }
            )
        }
    }
}