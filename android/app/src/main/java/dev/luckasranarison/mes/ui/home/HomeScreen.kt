package dev.luckasranarison.mes.ui.home

import android.net.Uri
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController
import dev.luckasranarison.mes.Activities
import dev.luckasranarison.mes.Routes
import dev.luckasranarison.mes.extra.createNewShortcut
import dev.luckasranarison.mes.vm.EmulatorViewModel
import dev.luckasranarison.mes.vm.RomLoadingState
import dev.luckasranarison.mes.ui.rom.RomContainer

@Composable
fun Home(viewModel: EmulatorViewModel, controller: NavHostController) {
    val ctx = LocalContext.current
    val romFiles by viewModel.romFiles
    val romLoadingState by viewModel.romLoadingState
    val romDirectory by viewModel.romDirectory.observeAsState()

    val loadRomFromFile = rememberLauncherForActivityResult(Activities.GET_CONTENT) { uri ->
        if (uri != null) viewModel.loadRomFromFile(ctx, uri)
    }

    val chooseRomDirectory = rememberLauncherForActivityResult(Activities.GET_DIRECTORY) { uri ->
        if (uri != null) viewModel.setRomDirectory(ctx, uri)
    }

    LaunchedEffect(romDirectory) {
        if (romDirectory != null) {
            viewModel.loadRomFromDirectory(ctx, Uri.parse(romDirectory))
        }
    }

    LaunchedEffect(romLoadingState) {
        if (romLoadingState is RomLoadingState.Success) {
            controller.navigate(Routes.EMULATOR)
            viewModel.setLoadStatus(RomLoadingState.None)
        }

        if (romLoadingState is RomLoadingState.Error) {
            val errorMessage = (romLoadingState as RomLoadingState.Error).message
            Toast.makeText(ctx, errorMessage, Toast.LENGTH_SHORT).show()
            viewModel.setLoadStatus(RomLoadingState.None)
        }
    }

    Scaffold(
        topBar = { HomeTopAppBar(controller = controller) },
        floatingActionButton = {
            FloatingButton(onClick = { loadRomFromFile.launch("application/octet-stream") })
        }
    ) { innerPadding ->
        when {
            romDirectory != null && romFiles == null -> { // Loading
                Box(
                    modifier = Modifier
                        .padding(innerPadding)
                        .fillMaxSize()
                ) {
                    CircularProgressIndicator(
                        modifier = Modifier.align(Alignment.Center),
                        color = MaterialTheme.colorScheme.primary
                    )
                }
            }

            romFiles == null -> DirectoryChooser(
                modifier = Modifier.padding(innerPadding),
                onChoose = { chooseRomDirectory.launch(null) }
            )

            else -> LazyColumn(modifier = Modifier.padding(innerPadding)) {
                items(romFiles!!.size) { index ->
                    val rom = romFiles!![index]
                    RomContainer(
                        rom = rom,
                        onSelect = { viewModel.loadRomFromFile(ctx, rom.uri) },
                        onCreateShortcut = { createNewShortcut(ctx, rom) }
                    )
                }
            }
        }
    }
}