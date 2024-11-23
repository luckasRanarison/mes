package dev.luckasranarison.mes.data

import android.util.Log
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class SettingsRepository(private val dataStore: DataStore<Preferences>) {
    object Keys {
        val ROM_DIR = stringPreferencesKey("rom_dir")
        val ENABLE_APU = booleanPreferencesKey("enable_apu")
    }

    suspend fun setRomDirectory(dir: String) {
        dataStore.edit { pref -> pref[Keys.ROM_DIR] = dir }
    }

    suspend fun toggleApuState() {
        dataStore.edit { pref -> pref[Keys.ENABLE_APU] = !(pref[Keys.ENABLE_APU] ?: true) }
    }

    fun getRomDirectory(): Flow<String?> {
        return dataStore.data.map { pref -> pref[Keys.ROM_DIR] }
    }

    fun getApuState(): Flow<Boolean?> {
        return dataStore.data.map { pref -> pref[Keys.ENABLE_APU] }
    }
}