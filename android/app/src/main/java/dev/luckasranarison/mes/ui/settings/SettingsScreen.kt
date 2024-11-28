package dev.luckasranarison.mes.ui.settings

import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.setValue
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
    val colorPalette by viewModel.colorPalette.observeAsState()
    var showPaletteOptions by remember { mutableStateOf(false) }

    val wrapBlock: (() -> Unit) -> Unit = { block ->
        try {
            block()
        } catch (err: Exception) {
            val message = err.message ?: "An unknown error occurred"
            Toast.makeText(ctx, message, Toast.LENGTH_SHORT).show()
        }
    }

    val chooseRomDirectory = rememberLauncherForActivityResult(Activities.GET_DIRECTORY) { uri ->
        if (uri != null) wrapBlock { viewModel.setRomDirectory(ctx, uri) }
    }

    val chooseColorPalette = rememberLauncherForActivityResult(Activities.GET_CONTENT) { uri ->
        if (uri != null) wrapBlock { viewModel.setColorPalette(ctx, uri) }

    }

    LaunchedEffect(colorPalette) {
        showPaletteOptions = colorPalette != null
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
                    label = "Custom palette",
                    description = "Use custom .pal palette",
                    value = showPaletteOptions,
                    onToggle = { value ->
                        showPaletteOptions = value
                        if (!value) viewModel.unsetColorPalette()
                    }
                )

                if (showPaletteOptions) {
                    TextValue(
                        label = "Palette",
                        value = "Custom palette file",
                        onChange = { chooseColorPalette.launch("*/*") }
                    )
                }

                BooleanValue(
                    label = "Sound",
                    description = "Enable APU emulation",
                    value = enableApu ?: true,
                    onToggle = { viewModel.toggleApuState() }
                )
            }
        }
    }
}