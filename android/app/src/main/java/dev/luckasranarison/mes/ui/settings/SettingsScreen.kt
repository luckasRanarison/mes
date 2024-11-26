package dev.luckasranarison.mes.ui.settings

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.Activities
import dev.luckasranarison.mes.vm.EmulatorViewModel

@Composable
fun Settings(viewModel: EmulatorViewModel, onExit: () -> Unit) {
    val ctx = LocalContext.current
    val romDirectory by viewModel.romDirectory.observeAsState()
    val enableApu by viewModel.enableApu.observeAsState()

    val chooseRomDirectory = rememberLauncherForActivityResult(Activities.GET_DIRECTORY) { uri ->
        if (uri != null) viewModel.setRomDirectory(ctx, uri)
    }

    Scaffold(
        topBar = {
            SettingsTopAppBar(onExit = onExit)
        },
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .verticalScroll(rememberScrollState())
                .fillMaxSize(),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            Section(header = "ROMs") {
                TextValue(
                    label = "Directory",
                    value = romDirectory?.toPathName(ctx) ?: "Unset",
                    onChange = { chooseRomDirectory.launch(null) }
                )
            }
            Section(header = "Emulator") {
                BooleanValue(
                    label = "Sound",
                    description = "Enable APU emulation",
                    value = enableApu ?: true,
                    onToggle = viewModel::toggleApuState
                )
            }
        }
    }
}