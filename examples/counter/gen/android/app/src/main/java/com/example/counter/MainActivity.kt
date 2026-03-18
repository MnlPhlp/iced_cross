package com.example.counter

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriNativeActivity() {
  // This is the main activity of the app, which extends TauriNativeActivity to allow for Java/Kotlin callbacks from native Rust code.
  // You  can add custom code here and call it from Rust using jni, but it might be easier to use the tauri plugin system  for native code.
}