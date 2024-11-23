package dev.luckasranarison.mes

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.viewModels
import androidx.compose.animation.ExitTransition
import androidx.compose.runtime.*
import androidx.navigation.compose.*
import dev.luckasranarison.mes.lib.Rust
import dev.luckasranarison.mes.ui.emulator.Emulator
import dev.luckasranarison.mes.ui.theme.MesTheme
import dev.luckasranarison.mes.ui.emulator.EmulatorViewModel
import dev.luckasranarison.mes.ui.home.Home
import dev.luckasranarison.mes.ui.settings.Settings

object Activities {
    val GET_CONTENT = ActivityResultContracts.GetContent()
    val GET_DIRECTORY = ActivityResultContracts.OpenDocumentTree()
}

class MainActivity : ComponentActivity() {
    private val viewModel: EmulatorViewModel by viewModels { EmulatorViewModel.Factory }

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
                App(viewModel = viewModel)
            }
        }
    }

    override fun onPause() {
        super.onPause()
        viewModel.pauseEmulation()
    }

    override fun onResume() {
        super.onResume()
        viewModel.startEmulation()
    }
}

@Composable
fun App(viewModel: EmulatorViewModel) {
    val navController = rememberNavController()
    val currentStack by navController.currentBackStackEntryAsState()

    NavHost(navController = navController, startDestination = Routes.HOME) {
        composable(Routes.HOME) {
            Home(viewModel = viewModel, controller = navController)
        }
        composable(Routes.EMULATOR, popExitTransition = { ExitTransition.None }) {
            Emulator(viewModel = viewModel, controller = navController)
        }
        composable(Routes.SETTINGS, popExitTransition = {
            if (currentStack?.destination?.route == Routes.EMULATOR) ExitTransition.None else null
        }) {
            Settings(viewModel = viewModel, controller = navController)
        }
    }
}