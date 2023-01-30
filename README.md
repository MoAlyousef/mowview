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
A- Assuming you're creating a pure Rust ios app:

1- Create a new Rust ios project:
```
cargo new myapp
```

2- Add dependencies in Cargo.toml
```toml
[dependencies]
mowview = "0.1"

[target.'cfg(target_os="ios")'.dependencies]
objc = "0.2.7"

[package.metadata.bundle]
name = "myapp"
identifier = "com.neurosrg.myapp"
category = "Education"
short_description = "A pure rust app"
long_description = "A pure rust app"
```
Notice the package.metadata.bundle heading, we'll use it to be able to use cargo-bundle.

3- Create a build.rs script:
```rust
fn main() {
    println!("cargo:rustc-link-lib=framework=UIKit");
    println!("cargo:rustc-link-lib=framework=WebKit");
}
```

4- A minimalist main.rs (still kinda verbose):
```rust
extern crate objc; // remember to add it to your Cargo.toml!

use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
struct Frame(pub f64, pub f64, pub f64, pub f64);

extern "C" fn did_finish_launching_with_options(
    obj: &mut Object,
    _: Sel,
    _: *mut Object,
    _: *mut Object,
) -> BOOL {
    unsafe {
        let frame: *mut Object = msg_send![class!(UIScreen), mainScreen];
        let frame: Frame = msg_send![frame, bounds];
        let win: *mut Object = msg_send![class!(UIWindow), alloc];
        let win: *mut Object = msg_send![win, initWithFrame: frame];
        let vc: *mut Object = msg_send![class!(ViewController), new];
        let _: () = msg_send![win, setRootViewController: vc];
        let _: () = msg_send![win, makeKeyAndVisible];
        let white: *mut Object = msg_send![class!(UIColor), whiteColor];
        let _: () = msg_send![win, setBackgroundColor: white];
        obj.set_ivar("window", win as usize);
    }
    YES
}

extern "C" fn did_load(obj: &mut Object, _: Sel) {
    unsafe {
        let _: () = msg_send![super(obj, class!(UIViewController)), viewDidLoad];
        let frame: *mut Object = msg_send![class!(UIScreen), mainScreen];
        let frame: Frame = msg_send![frame, bounds];
        let view: *mut Object = msg_send![&*obj, view];
        let webview: *mut Object = msg_send![class!(WKWebView), alloc];
        let webview: *mut Object = msg_send![webview, initWithFrame: frame];
        let _: () = msg_send![view, addSubview: webview];
        let webview = mowview::WebView::from(webview as _);
        webview.load_url("https://www.google.com");
    }
}

fn main() {
    unsafe {
        let ui_responder_cls = class!(UIResponder);
        let mut app_delegate_cls = ClassDecl::new("AppDelegate", ui_responder_cls).unwrap();

        app_delegate_cls.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            did_finish_launching_with_options
                as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object) -> BOOL,
        );

        app_delegate_cls.add_ivar::<usize>("window");

        app_delegate_cls.register();

        let ui_view_controller_cls = class!(UIViewController);
        let mut view_controller_cls =
            ClassDecl::new("ViewController", ui_view_controller_cls).unwrap();

        view_controller_cls.add_method(
            sel!(viewDidLoad),
            did_load as extern "C" fn(&mut Object, Sel),
        );

        view_controller_cls.add_method(sel!(clicked), clicked as extern "C" fn(&mut Object, Sel));

        view_controller_cls.register();

        let name: *mut Object =
            msg_send![class!(NSString), stringWithUTF8String:"AppDelegate\0".as_ptr()];

        extern "C" {
            fn UIApplicationMain(
                argc: i32,
                argv: *mut *mut c_char,
                principalClass: *mut Object,
                delegateName: *mut Object,
            ) -> i32;
        }

        let autoreleasepool: *mut Object = msg_send![class!(NSAutoreleasePool), new];
        // Anything needing the autoreleasepool
        let _: () = msg_send![autoreleasepool, drain];

        UIApplicationMain(0, ptr::null_mut(), ptr::null_mut(), name);
    }
}
```
Notice that we create our webview in the did_load method.

5- Run cargo-bundle (install it first if it wasn't already installed):
```
cargo bundle --target x86_64-apple-ios
```

6- Boot a simulator:
```
xcrun simctl list # to get list of simulators
xcrun simctl boot "iPhone 13 Pro" # to start my iphone 13 pro simulator
```

7- Install into simulator:
```
xcrun simctl install booted target/x86_64-apple-ios/debug/bundle/ios/myapp.app
```

8- Launch app from the simulator

B- Using XCode:
- You'll have to create an Objective-C or Swift iOS application, and a Rust library (either static or cdylib).
- Add it to your depenencies, as well as WebKit framework.
- Like in Rust, you'll want to either create the webview programmatically or using storyboards.
- Before didLoad, declare an extern function that will take the webview as a void pointer:
```
// objective-c
extern void myHandleWebView(void *webviewptr);
```
- In didLoad, call `myHandleWebView(webview);`. Depending on the resource management (ARC or not), you might have to cast using `(void *)CFBridgingRetain(webview)`.
