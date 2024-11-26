package dev.luckasranarison.mes

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.viewModels
import dev.luckasranarison.mes.lib.Rust
import dev.luckasranarison.mes.ui.theme.MesTheme
import dev.luckasranarison.mes.vm.EmulatorViewModel

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