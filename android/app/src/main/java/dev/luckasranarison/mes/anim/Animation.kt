package dev.luckasranarison.mes.anim

import androidx.compose.animation.EnterTransition
import androidx.compose.animation.ExitTransition
import androidx.compose.animation.core.FastOutSlowInEasing
import androidx.compose.animation.core.FiniteAnimationSpec
import androidx.compose.animation.core.tween
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.ui.unit.IntOffset

object Animations {
    private val animationSpec: FiniteAnimationSpec<IntOffset> = tween(
        durationMillis = 300,
        easing = FastOutSlowInEasing
    )

    val EnterTransition: EnterTransition = slideInHorizontally(
        initialOffsetX = { it },
        animationSpec = animationSpec
    )

    val ExitTransition: ExitTransition = slideOutHorizontally(
        targetOffsetX = { -it },
        animationSpec = animationSpec
    )

    val PopEnterTransition: EnterTransition = slideInHorizontally(
        initialOffsetX = { -it },
        animationSpec = animationSpec
    )

    val PopExitTransition: ExitTransition = slideOutHorizontally(
        targetOffsetX = { it },
        animationSpec = animationSpec
    )
}