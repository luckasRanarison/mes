package dev.luckasranarison.mes.lib

import kotlin.experimental.and
import kotlin.experimental.inv
import kotlin.experimental.or

class Controller(private var value: Byte = 0b0000_0000) {
    fun update(button: Button, state: Boolean): Controller {
        val bits = (1 shl button.ordinal).toByte()
        val value = if (state) value or bits else value and bits.inv()
        return Controller(value)
    }

    fun state(): Byte = value
}

enum class Button { Right, Left, Down, Up, Start, Select, B, A }