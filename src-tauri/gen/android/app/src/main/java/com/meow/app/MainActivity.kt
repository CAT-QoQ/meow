package com.meow.app

import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.widget.Toast
import android.os.Environment
import androidx.annotation.RequiresApi
import android.view.View
import android.view.WindowManager
import android.graphics.Color
import android.view.WindowInsetsController

class MainActivity : TauriActivity() {

    companion object {
        private var instance: MainActivity? = null

        @JvmStatic
        fun getInstance(): MainActivity {
            return instance ?: throw IllegalStateException("MainActivity not initialized")
        }

        @JvmStatic
        external fun setRustJNIEnv()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        instance = this

        // 初始化 Rust 的 JNI 环境
        setRustJNIEnv()

        // 实现透明状态栏
        makeStatusBarTransparent()

        // 检查并请求 MANAGE_EXTERNAL_STORAGE 权限
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            if (!Environment.isExternalStorageManager()) {
                // 引导用户前往设置页面授予权限
                val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION)
                intent.data = Uri.parse("package:$packageName")
                startActivityForResult(intent, 100)
            } else {
                // 已经获得权限，可以继续您的操作
                Toast.makeText(this, "已获得全部文件访问权限", Toast.LENGTH_SHORT).show()
            }
        } else {
            // 针对 Android 11 以下的设备，请求普通的存储权限
            requestLegacyStoragePermission()
        }
        setRustJNIEnv()
    }

    // 方法：实现透明状态栏
    private fun makeStatusBarTransparent() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            window.decorView.systemUiVisibility = (View.SYSTEM_UI_FLAG_LAYOUT_STABLE or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN)
            window.statusBarColor = Color.TRANSPARENT
        } else {
            window.setFlags(
                WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS,
                WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS
            )
        }
    }

    // 请求 Android 10 及以下版本的存储权限
    private fun requestLegacyStoragePermission() {
        // 您可以在这里添加请求 READ_EXTERNAL_STORAGE 和 WRITE_EXTERNAL_STORAGE 的代码
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            requestPermissions(arrayOf(
                android.Manifest.permission.READ_EXTERNAL_STORAGE,
                android.Manifest.permission.WRITE_EXTERNAL_STORAGE
            ), 101)
        }
    }

    @RequiresApi(Build.VERSION_CODES.R)
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == 100) {
            if (Environment.isExternalStorageManager()) {
                // 用户已授予权限
                Toast.makeText(this, "权限授予成功", Toast.LENGTH_SHORT).show()
            } else {
                // 用户未授予权限
                Toast.makeText(this, "权限授予失败", Toast.LENGTH_SHORT).show()
            }
        }
    }

    fun toggleStatusBarTextColor(isDarkMode: Boolean): Boolean {
        try {
            val window = this.window ?: throw NullPointerException("Window is null")
            val decorView = window.decorView ?: throw NullPointerException("DecorView is null")

            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R) {
                // Android 11 (API 30) 及以上版本
                val controller = decorView.windowInsetsController
                    ?: throw NullPointerException("WindowInsetsController is null")
                if (isDarkMode) {
                    controller.setSystemBarsAppearance(
                        android.view.WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS,
                        android.view.WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
                    )
                } else {
                    controller.setSystemBarsAppearance(
                        0,
                        android.view.WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
                    )
                }
            } else if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
                // Android 6.0 到 Android 10
                val flags = decorView.systemUiVisibility
                if (isDarkMode) {
                    decorView.systemUiVisibility = flags or android.view.View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR
                } else {
                    decorView.systemUiVisibility = flags and android.view.View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR.inv()
                }
            } else {
                // Android 6.0 以下版本不支持状态栏文字颜色切换
                return false
            }

            return true
        } catch (e: Exception) {
            // 捕获异常并记录日志
            android.util.Log.e("MainActivity", "Error in yourMethodName", e)
            throw e
        }
    }
}
