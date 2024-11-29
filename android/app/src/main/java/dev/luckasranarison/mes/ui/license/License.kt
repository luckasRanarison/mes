package dev.luckasranarison.mes.ui.license

import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController
import com.mikepenz.aboutlibraries.ui.compose.m3.rememberLibraries
import dev.luckasranarison.mes.ui.shared.GenericTopAppBar
import dev.luckasranarison.mes.R

@Composable
fun License(controller: NavHostController) {
    val ctx = LocalContext.current

    val libs by rememberLibraries {
        ctx.resources
            .openRawResource(R.raw.aboutlibraries)
            .readBytes()
            .decodeToString()
    }

    Scaffold(
        topBar = {
            GenericTopAppBar(
                title = "Open source license",
                onExit = { controller.popBackStack() }
            )
        }
    ) { innerPadding ->
        LazyColumn(modifier = Modifier.padding(innerPadding)) {
            items(libs?.libraries?.size ?: 0) { index ->
                LibraryContainer(libs!!.libraries[index])
            }
        }
    }
}