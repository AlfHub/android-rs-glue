#![feature(macro_rules)]
#![feature(phase)]

#![unstable]

#[phase(plugin)] extern crate compile_msg;
extern crate libc;

mod ffi;

static mut native_window: ffi::NativeWindowType = 0 as ffi::NativeWindowType;

#[cfg(not(target_os = "android"))]
compile_note!("You are not compiling for Android")


macro_rules! android_start(
    ($main: ident) => (
        pub mod __android_start {
            extern crate android_glue;

            /// This is the entry point of the Android application
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn ANativeActivity_onCreate(activity: *mut ffi::ANativeActivity,
                _saved_state: *mut libc::c_void, _saved_state_size: libc::size_t)
            {
                use std::mem;
                use std::task::TaskBuilder;

                let mut activity = unsafe { &mut *activity };
                let mut callbacks = unsafe { &mut *activity.callbacks };

                callbacks.onDestroy = android_glue::native_ondestroy;
                callbacks.onStart = android_glue::native_onstart;
                callbacks.onResume = android_glue::native_onresume;
                callbacks.onSaveInstanceState = android_glue::native_onsaveinstancestate;
                callbacks.onPause = android_glue::native_onpause;
                callbacks.onStop = android_glue::native_onstop;
                callbacks.onConfigurationChanged = android_glue::native_onconfigurationchanged;
                callbacks.onLowMemory = android_glue::native_onlowmemory;
                callbacks.onWindowFocusChanged = android_glue::native_onwindowfocuschanged;
                callbacks.onNativeWindowCreated = android_glue::native_onnativewindowcreated;
                callbacks.onNativeWindowDestroyed = android_glue::native_onnativewindowdestroyed;

                TaskBuilder::new().native().spawn(proc() {
                    $main()
                });
            }
        }
    )
)

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onnativewindowcreated(_: *mut ffi::ANativeActivity, window: *const ffi::ANativeWindow) {
    unsafe { native_window = window; }
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onnativewindowdestroyed(_: *mut ffi::ANativeActivity, _: *const ffi::ANativeWindow) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_ondestroy(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onstart(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onresume(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onsaveinstancestate(_: *mut ffi::ANativeActivity, _: *mut libc::size_t) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onpause(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onstop(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onconfigurationchanged(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onlowmemory(_: *mut ffi::ANativeActivity) {
}

#[doc(hidden)]
#[allow(visible_private_types)]
pub extern fn native_onwindowfocuschanged(_: *mut ffi::ANativeActivity, _: libc::c_int) {
}
