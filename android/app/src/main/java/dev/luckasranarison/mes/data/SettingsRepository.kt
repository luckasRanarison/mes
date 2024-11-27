package dev.luckasranarison.mes.data

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.core.byteArrayPreferencesKey
import kotlinx.coroutines.flow.map

class SettingsRepository(private val dataStore: DataStore<Preferences>) {
    object Keys {
        val ROM_DIR = stringPreferencesKey("rom_dir")
        val ENABLE_APU = booleanPreferencesKey("enable_apu")
        val COLOR_PALETTE = byteArrayPreferencesKey("color_palette")
    }

    suspend fun setRomDirectory(dir: String) {
        dataStore.edit { pref -> pref[Keys.ROM_DIR] = dir }
    }

    suspend fun toggleApuState() {
        dataStore.edit { pref -> pref[Keys.ENABLE_APU] = !(pref[Keys.ENABLE_APU] ?: true) }
    }

    suspend fun setColorPalette(palette: ByteArray?) {
        if (palette != null) {
            dataStore.edit { pref -> pref[Keys.COLOR_PALETTE] = palette }
        } else {
            dataStore.edit { pref -> pref.remove(Keys.COLOR_PALETTE) }
        }
    }

    fun getRomDirectory() = dataStore.data.map { pref -> pref[Keys.ROM_DIR] }
    fun getApuState() = dataStore.data.map { pref -> pref[Keys.ENABLE_APU] }
    fun getColorPalette() = dataStore.data.map { pref -> pref[Keys.COLOR_PALETTE] }
}