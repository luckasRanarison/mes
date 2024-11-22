package dev.luckasranarison.mes.lib

import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioManager
import android.media.AudioTrack

const val sampleRate = 44100
const val channelConfig = AudioFormat.CHANNEL_OUT_MONO
const val audioFormat = AudioFormat.ENCODING_PCM_FLOAT

val minBufferSize = AudioTrack.getMinBufferSize(sampleRate, channelConfig, audioFormat)

fun createAudioTrack(): AudioTrack {
    return AudioTrack(
        AudioAttributes.Builder()
            .setUsage(AudioAttributes.USAGE_MEDIA)
            .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
            .build(),
        AudioFormat.Builder()
            .setSampleRate(sampleRate)
            .setChannelMask(channelConfig)
            .setEncoding(audioFormat)
            .build(),
        minBufferSize,
        AudioTrack.MODE_STREAM,
        AudioManager.AUDIO_SESSION_ID_GENERATE
    )
}