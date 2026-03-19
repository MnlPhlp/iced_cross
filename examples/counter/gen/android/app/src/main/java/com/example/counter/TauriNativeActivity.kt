package com.example.counter

import android.content.Intent
import android.content.res.Configuration
import app.tauri.plugin.PluginManager
import com.google.androidgamesdk.GameActivity

/**
 * Custom MainActivity that extends GameActivity. This allows us to use native rendering and still
 * support tauri plugins
 */
abstract class TauriNativeActivity : GameActivity() {
    val pluginManager = PluginManager(this)

    fun getAppClass(name: String): Class<*> {
        return Class.forName(name)
    }

    private external fun processMainThreadQueueNative()

    fun processMainThreadQueue() {
        runOnUiThread { processMainThreadQueueNative() }
    }

    override fun onDestroy() {
        super.onDestroy()
        pluginManager.onDestroy()
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        pluginManager.onNewIntent(intent)
    }

    override fun onResume() {
        super.onResume()
        pluginManager.onResume()
    }

    override fun onPause() {
        super.onPause()
        pluginManager.onPause()
    }

    override fun onRestart() {
        super.onRestart()
        pluginManager.onRestart()
    }

    override fun onStop() {
        super.onStop()
        pluginManager.onStop()
    }

    override fun onConfigurationChanged(newConfig: Configuration) {
        super.onConfigurationChanged(newConfig)
        pluginManager.onConfigurationChanged(newConfig)
    }
}
