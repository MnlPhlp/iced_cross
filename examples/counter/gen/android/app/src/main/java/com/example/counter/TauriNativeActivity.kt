package de.philipp_manuel.slint_android_test

import android.app.NativeActivity
import app.tauri.plugin.PluginManager

class CompatActivity : TauriActivity() {

}

/**
 * Custom MainActivity that extends NativeActivity.
 * This allows us to handle Java/Kotlin callbacks from native Rust code.
 */
abstract class TauriNativeActivity : NativeActivity() {
    val compat = CompatActivity()
    var pluginManager: PluginManager = PluginManager(compat) 
    fun getAppClass(name: String): Class<*> {
        return Class.forName(name)
    }
}
