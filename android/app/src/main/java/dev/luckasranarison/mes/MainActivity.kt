package dev.luckasranarison.mes

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.viewModels
import androidx.compose.runtime.*
import androidx.navigation.compose.*
import dev.luckasranarison.mes.lib.Rust
import dev.luckasranarison.mes.ui.emulator.Emulator
import dev.luckasranarison.mes.ui.theme.MesTheme
import dev.luckasranarison.mes.ui.emulator.EmulatorViewModel
import dev.luckasranarison.mes.ui.emulator.RomLoadingState
import dev.luckasranarison.mes.ui.home.Home
import dev.luckasranarison.mes.ui.system.ScreenEffect
import java.io.IOException


class MainActivity : ComponentActivity() {
    val emulatorViewModel by viewModels<EmulatorViewModel>()

    companion object {
        init {
            System.loadLibrary("mes_jni")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        Rust.setPanicHook() // redirect rust panic output to Logcat
        enableEdgeToEdge()
        setContent { MesTheme { App(ctx = this) } }
    }

    override fun onPause() {
        super.onPause()
        emulatorViewModel.pauseEmulation()
    }

    override fun onResume() {
        super.onResume()
        emulatorViewModel.resumeEmulation()
    }

    val pickFile = registerForActivityResult(ActivityResultContracts.GetContent()) { uri ->
        try {
            if (uri == null) throw IOException("File URI is null")

            val stream = contentResolver.openInputStream(uri)

            stream.use { handle ->
                val rom = handle?.readBytes() ?: throw IOException("Failed to read ROM from file")
                emulatorViewModel.setRom(rom)
                emulatorViewModel.setLoadSuccess()
            }
        } catch (err: Exception) {
            emulatorViewModel.setLoadError(err)
        }
    }
}

@Composable
fun App(ctx: MainActivity) {
    val navController = rememberNavController()
    val romLoadingState by ctx.emulatorViewModel.romLoadingState

    LaunchedEffect(romLoadingState) {
        if (romLoadingState is RomLoadingState.Success) navController.navigate("emulator")
    }

    ScreenEffect(navController = navController)

    NavHost(navController = navController, startDestination = "home") {
        composable("home") {
            Home(
                pickFile = { ctx.pickFile.launch("*/*") },
                controller = navController
            )
        }
        composable("emulator") {
            Emulator(viewModel = ctx.emulatorViewModel)
        }
    }
}