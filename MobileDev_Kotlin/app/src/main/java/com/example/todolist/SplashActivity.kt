package com.example.todolist

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle

class SplashActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_splash)
        getSupportActionBar().hide();

        final Intent i = new Intent(packageContext: SplashActivity.this.MainActivity.class);
        new Handler().postDelayed(new Runnable(){
            @Override
            public void run(){
                startActivity(i);
                finish();
            }
        }, delaymillis: 1000);
    }
}