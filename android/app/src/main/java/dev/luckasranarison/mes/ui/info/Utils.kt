package dev.luckasranarison.mes.ui.info

import android.content.Context

fun getAppVersion(ctx: Context) =
    ctx.packageManager?.getPackageInfo(ctx.packageName, 0)?.versionName
        ?: throw Exception("Failed to get package info")

fun makeMailMessage(address: String) =
    "https://mail.google.com/mail/?view=cm&fs=1&to=$address&su=Subject&body=Message"