package dev.luckasranarison.mes.ui.license

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.mikepenz.aboutlibraries.entity.Library
import dev.luckasranarison.mes.R

@Composable
@OptIn(ExperimentalMaterial3Api::class)
fun BottomSheet(library: Library, onClose: () -> Unit) {
    val uriHandler = LocalUriHandler.current
    val sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)

    ModalBottomSheet(
        onDismissRequest = { onClose() },
        sheetState = sheetState,
        containerColor = MaterialTheme.colorScheme.surface,
    ) {
        Column(
            modifier = Modifier
                .fillMaxHeight(0.6f)
                .verticalScroll(rememberScrollState())
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    Text(text = library.name, fontWeight = FontWeight.SemiBold)
                    Text(
                        text = "v${library.artifactVersion}",
                        color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.6f)
                    )
                }

                if (library.website != null) {
                    IconButton(onClick = { uriHandler.openUri(library.website!!) }) {
                        Icon(
                            painter = painterResource(id = R.drawable.web_search),
                            contentDescription = "Website"
                        )
                    }
                }
            }

            Text(
                text = "Description: ${library.description ?: "No description"}",
                modifier = Modifier.padding(16.dp)
            )

            HorizontalDivider(thickness = 1.dp)

            library.licenses.forEach {
                Text(
                    text = it.licenseContent ?: "No license",
                    modifier = Modifier.padding(16.dp)
                )
            }
        }
    }
}