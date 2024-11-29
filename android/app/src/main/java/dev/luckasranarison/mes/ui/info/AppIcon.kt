package dev.luckasranarison.mes.ui.info

import androidx.compose.foundation.layout.*
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import dev.luckasranarison.mes.R

@Composable
fun AppIcon() {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.Center
    ) {
        Icon(
            painter = painterResource(id = R.drawable.nes_icon),
            contentDescription = "Icon",
            tint = MaterialTheme.colorScheme.onBackground,
            modifier = Modifier
                .padding(48.dp)
                .size(82.dp),
        )
    }
}