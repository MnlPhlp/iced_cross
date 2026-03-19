package com.example.counter

import android.app.Activity
import android.app.NativeActivity
import androidx.appcompat.app.AppCompatActivity
import app.tauri.plugin.PluginManager

class CompatActivity : AppCompatActivity() {
    val pluginManager = PluginManager(this)

    fun getAppClass(name: String): Class<*> {
        return Class.forName(name)
    }

    fun forwardCreate(savedInstanceState: android.os.Bundle?) {
        super.onCreate(savedInstanceState)
    }
}

/**
 * Custom MainActivity that extends NativeActivity. This allows us to handle Java/Kotlin callbacks
 * from native Rust code.
 */
abstract class TauriNativeActivity : NativeActivity() {
    val compat = CompatActivity()

    fun getCompatActivity(): Activity {
        return this.compat
    }

    override fun onCreate(savedInstanceState: android.os.Bundle?) {
        super.onCreate(savedInstanceState)
        compat.forwardCreate(savedInstanceState)
    }
}
