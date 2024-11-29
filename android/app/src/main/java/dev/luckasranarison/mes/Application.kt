package dev.luckasranarison.mes

import androidx.compose.animation.EnterTransition
import androidx.compose.animation.ExitTransition
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import dev.luckasranarison.mes.anim.Animations
import dev.luckasranarison.mes.ui.emulator.Emulator
import dev.luckasranarison.mes.vm.EmulatorViewModel
import dev.luckasranarison.mes.ui.home.Home
import dev.luckasranarison.mes.ui.info.Info
import dev.luckasranarison.mes.ui.license.License
import dev.luckasranarison.mes.ui.settings.Settings

data object Routes {
    const val HOME = "home"
    const val EMULATOR = "emulator"
    const val SETTINGS = "settings"
    const val INFO = "info"
    const val LICENSE = "licenses"
}

@Composable
fun App(viewModel: EmulatorViewModel) {
    val navController = rememberNavController()
    val isShortcutLaunch by remember { viewModel.isShortcutLaunch }

    NavHost(
        navController = navController,
        startDestination = if (isShortcutLaunch) Routes.EMULATOR else Routes.HOME,
        enterTransition = { Animations.EnterTransition },
        exitTransition = { Animations.ExitTransition },
        popEnterTransition = { Animations.PopEnterTransition },
        popExitTransition = { Animations.PopExitTransition }
    ) {
        composable(Routes.HOME) {
            Home(viewModel = viewModel, controller = navController)
        }
        composable(Routes.EMULATOR, popExitTransition = { ExitTransition.None }) {
            Emulator(viewModel = viewModel, controller = navController)
        }
        composable(Routes.INFO) {
            Info(controller = navController)
        }
        composable(Routes.LICENSE) {
            License(controller = navController)
        }
        composable(Routes.SETTINGS, enterTransition = {
            if (initialState.destination.route == Routes.EMULATOR) EnterTransition.None else null
        }) {
            Settings(viewModel = viewModel, onExit = navController::popBackStack)
        }
    }
}