package dev.luckasranarison.mes

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.viewModels
import androidx.compose.animation.ExitTransition
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.compose.*
import dev.luckasranarison.mes.lib.Rust
import dev.luckasranarison.mes.ui.emulator.Emulator
import dev.luckasranarison.mes.ui.theme.MesTheme
import dev.luckasranarison.mes.ui.emulator.EmulatorViewModel
import dev.luckasranarison.mes.ui.emulator.RomLoadingState
import dev.luckasranarison.mes.ui.home.Home
import java.io.IOException


class MainActivity : ComponentActivity() {
    private val emulatorViewModel by viewModels<EmulatorViewModel>()

    companion object {
        init {
            System.loadLibrary("mes_jni")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        Rust.setPanicHook() // Redirects Rust panics output to Log before crashing

        enableEdgeToEdge()
        setContent {
            MesTheme {
                App(viewModel = emulatorViewModel, pickFile = pickFile)
            }
        }
    }

    override fun onPause() {
        super.onPause()
        emulatorViewModel.pauseEmulation()
    }

    override fun onResume() {
        super.onResume()
        emulatorViewModel.startEmulation()
    }

    private val pickFile = registerForActivityResult(ActivityResultContracts.GetContent()) { uri ->
        try {
            if (uri == null) return@registerForActivityResult

            contentResolver.openInputStream(uri).use { handle ->
                val rom = handle?.readBytes() ?: throw IOException("Failed to read ROM from file")
                emulatorViewModel.setRom(rom)
                emulatorViewModel.setLoadStatus(RomLoadingState.Success)
            }
        } catch (err: Exception) {
            val message = err.message ?: "An unknown error occurred"
            emulatorViewModel.setLoadStatus(RomLoadingState.Error(message))
        }
    }
}

@Composable
fun App(viewModel: EmulatorViewModel, pickFile: ActivityResultLauncher<String>) {
    val ctx = LocalContext.current
    val navController = rememberNavController()
    val romLoadingState by viewModel.romLoadingState

    LaunchedEffect(romLoadingState) {
        when (romLoadingState) {
            is RomLoadingState.None -> {}
            is RomLoadingState.Success -> navController.navigate(Routes.EMULATOR)
            is RomLoadingState.Error -> {
                val errorMessage = (romLoadingState as RomLoadingState.Error).message
                Toast.makeText(ctx, errorMessage, Toast.LENGTH_SHORT).show()
            }
        }

        viewModel.setLoadStatus(RomLoadingState.None)
    }

    NavHost(
        navController = navController,
        startDestination = Routes.HOME,
        popExitTransition = { ExitTransition.None }
    ) {
        composable(Routes.HOME) {
            Home(
                pickFile = { pickFile.launch("*/*") },
                controller = navController
            )
        }
        composable(Routes.EMULATOR) {
            Emulator(viewModel = viewModel, controller = navController)
        }
    }
}