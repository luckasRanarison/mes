import com.android.build.gradle.internal.tasks.factory.dependsOn
import java.nio.file.Path

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.kotlin.serialization)
}

android {
    namespace = "dev.luckasranarison.mes"
    compileSdk = 34

    defaultConfig {
        applicationId = "dev.luckasranarison.mes"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    kotlinOptions {
        jvmTarget = "11"
    }

    buildFeatures {
        compose = true
    }

    project.tasks.preBuild.dependsOn("buildRust")
}

dependencies {
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.activity.compose)
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.ui)
    implementation(libs.androidx.ui.graphics)
    implementation(libs.androidx.ui.tooling.preview)
    implementation(libs.androidx.material3)
    implementation(libs.androidx.navigation.compose)
    implementation(libs.androidx.runtime.livedata)
    implementation(libs.androidx.documentfile)
    implementation(libs.androidx.datastore.preferences)
    implementation(libs.kotlinx.serialization.core)
    implementation(libs.kotlinx.serialization.json)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.ui.test.junit4)
    debugImplementation(libs.androidx.ui.tooling)
    debugImplementation(libs.androidx.ui.test.manifest)
}

tasks.register("buildRust") {
    group = "build setup"
    description = "Builds the Rust shared library used with JNI"

    dependsOn("buildRustx86_64")
    dependsOn("buildRustArm64")
}

tasks.register("buildRustArm64") {
    group = "build setup"
    description = "Builds the Rust shared library for arm64"

    buildRustLibrary("aarch64", "arm64-v8a")
}

tasks.register("buildRustx86_64") {
    group = "build setup"
    description = "Builds the Rust shared library for x86_64"

    buildRustLibrary("x86_64")
}

fun buildRustLibrary(rustArch: String, directoryArch: String? = null) {
    val scriptFile = project.buildscript.sourceFile!!
    val parentPath = Path.of(scriptFile.parent!!)
    val libPath = parentPath.resolve("src/main/jniLibs")
    val buildPath = parentPath.resolve("../../target/$rustArch-linux-android/release/libmes_jni.so")
    val archLibPath = libPath.resolve(directoryArch ?: rustArch)

    exec {
        commandLine(
            "cargo",
            "build",
            "-p=mes-jni",
            "--target=$rustArch-linux-android",
            "--release"
        )
        commandLine("cp", buildPath, archLibPath)
    }
}