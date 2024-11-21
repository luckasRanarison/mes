package dev.luckasranarison.mes.ui.emulator

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Paint
import android.view.View
import androidx.core.graphics.scale
import dev.luckasranarison.mes.lib.SCREEN_HEIGHT
import dev.luckasranarison.mes.lib.SCREEN_WIDTH

class EmulatorView(context: Context) : View(context) {
    private val screen: Bitmap =
        Bitmap.createBitmap(SCREEN_WIDTH, SCREEN_HEIGHT, Bitmap.Config.ARGB_8888)

    init {
        val pixels = Array(SCREEN_WIDTH * SCREEN_HEIGHT) { 0xFFFFFFFF.toInt() }
        screen.setPixels(pixels.toIntArray(), 0, SCREEN_WIDTH, 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)

        val viewWidth = width.toFloat()
        val viewHeight = height.toFloat()

        val aspectRatio = SCREEN_HEIGHT.toFloat() / SCREEN_WIDTH.toFloat()
        val scaledWidth = viewHeight / aspectRatio
        val scaledBitmap = screen.scale(scaledWidth.toInt(), viewHeight.toInt(), false)
        val left = (viewWidth - scaledWidth) / 2

        canvas.drawBitmap(scaledBitmap, left, 0f, Paint())
    }

    fun updateScreenData(buffer: IntArray) {
        screen.setPixels(buffer, 0, SCREEN_WIDTH, 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)
        invalidate()
    }
}