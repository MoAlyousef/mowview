# mowview
A mobile webview crate.

## Instructions for Android:
1- Launch Android Studio

2- Create a new project choosing: Empty Activity. Also change the layout to Project instead of Android.

3- Add in app/src/main/AndroidManifest.xml:
```xml
<uses-permission android:name="android.permission.INTERNET" />
```

4- Add the rust plugin in build.gradle:
```groovy
plugins {
    // other plugins
    id 'org.mozilla.rust-android-gradle.rust-android' version '0.9.2' apply false
}
```

4- Add in app/build.gradle:
```groovy
plugins {
    id 'com.android.application'
    id 'org.mozilla.rust-android-gradle.rust-android'
}
```
5- In the same build.gradle file, append:
```groovy
cargo {
    module = "src/main/rust"
    libname = "rust"
    targets = ["arm", "arm64", "x86", "x86_64"] // choose the targets you want
    // Build Types should have "debug" in the name to invoke debug build of the Rust code
    // otherwise it defaults to "release" build
    profile = gradle.startParameter.taskNames.any{it.toLowerCase().contains("debug")} ? "debug" : "release"
}

afterEvaluate {
    // The `cargoBuild` task isn't available until after evaluation.
    android.applicationVariants.all { variant ->
        def productFlavor = ""
        variant.productFlavors.each {
            productFlavor += "${it.name.capitalize()}"
        }
        def buildType = "${variant.buildType.name.capitalize()}"
        tasks["generate${productFlavor}${buildType}Assets"].dependsOn(tasks["cargoBuild"])
    }
}
```

6- In MainActivity.java (under package line):
```java
import androidx.appcompat.app.AppCompatActivity;
import androidx.constraintlayout.widget.ConstraintLayout;

import android.os.Bundle;
import android.view.View;
import android.webkit.WebSettings;
import android.webkit.WebView;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("rust");
    }
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        ConstraintLayout layout = new ConstraintLayout(this);
        setContentView(layout);
        var webView = new WebView(this);
        layout.addView(webView);
        WebSettings webSettings = webView.getSettings();
        webSettings.setJavaScriptEnabled(true);
        handleWebView(webView);
    }

    public native void handleWebView(View view);
}
```

7- Init a rust directory in app/src/main (i.e. next to the java dir):
```bash
cd app/src/main
cargo new rust --lib
```

8- Change your Rust lib type to cdylib:
```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
mowview = "0.1"

[target.'cfg(target_os="android")'.dependencies]
jni = {version = "0.20", default-features = false}
```

9- In src/lib.rs
```rust
#[cfg(target_os = "android")]
use jni::{objects::{JClass, JObject}, JNIEnv};

#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_empt_MainActivity_handleWebView(
    env: JNIEnv,
    _: JClass,
    view: JObject,
) {
    let global = env.new_global_ref(view).unwrap();
    let webview = mowview::WebView::from(env, global.as_obj().into_raw() as _);
    webview.load_url("https://www.google.com");
}
```

## Instructions for iOS