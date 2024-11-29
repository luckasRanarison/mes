package dev.luckasranarison.mes.ui.license

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.mikepenz.aboutlibraries.entity.Library
import com.mikepenz.aboutlibraries.ui.compose.m3.util.author
import dev.luckasranarison.mes.ui.theme.Typography

@Composable
fun LibraryContainer(lib: Library) {
    var showSheet by remember { mutableStateOf(false) }

    if (showSheet) {
        BottomSheet(library = lib, onClose = { showSheet = false })
    }

    Box(modifier = Modifier
        .fillMaxWidth()
        .clickable { showSheet = true }
    ) {
        Row(
            modifier = Modifier
                .padding(horizontal = 16.dp, vertical = 12.dp)
                .fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
                Text(text = lib.name, maxLines = 1, overflow = TextOverflow.Ellipsis)
                Text(
                    text = lib.author,
                    style = Typography.titleSmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.6f)
                )
            }
            Text(text = lib.artifactVersion ?: "Unknown")
        }
    }
}