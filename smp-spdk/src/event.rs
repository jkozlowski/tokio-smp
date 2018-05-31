use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

#[allow(non_upper_case_globals, non_camel_case_types, unused)]
mod spdk_event {
    include!(concat!(env!("OUT_DIR"), "/spdk_event_bindings.rs"));
}

use self::spdk_event::{spdk_app_opts, spdk_app_opts_init, spdk_app_start};

pub struct AppOpts(spdk_app_opts);

impl AppOpts {
    pub fn new() -> Self {
        let mut opts: spdk_app_opts = Default::default();
        unsafe {
            spdk_app_opts_init(&mut opts as *mut spdk_app_opts);
        }
        AppOpts(opts)
    }

    pub fn name(&mut self, name: &'static str) {
        self.0.name = CString::new(name)
            .expect("Couldn't create a string")
            .into_raw()
    }

    pub fn config_file(&mut self, config_file: &'static str) {
        self.0.config_file = CString::new(config_file)
            .expect("Couldn't create a string")
            .into_raw()
    }

    pub fn start(mut self) -> Result<(), ()> {
        let ret = unsafe {
            let self_ref = &mut self;
            let opts_ref = &mut self_ref.0;
            spdk_app_start(
                opts_ref as *mut spdk_app_opts,
                Some(apply),
                // For now nothing to pass around
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        // pub type spdk_event_fn = :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut :: std :: os :: raw :: c_void , arg2 : * mut :: std :: os :: raw :: c_void ) > ;
        // pub fn spdk_app_start ( opts : * mut spdk_app_opts , start_fn : spdk_event_fn , arg1 : * mut :: std :: os :: raw :: c_void , arg2 : * mut :: std :: os :: raw :: c_void ) -> :: std :: os :: raw :: c_int ; }
        Ok(())
    }
}

impl Drop for AppOpts {
    fn drop(&mut self) {
        drop_if_not_null(self.0.name as *mut c_char);
        drop_if_not_null(self.0.config_file as *mut c_char);
    }
}

fn drop_if_not_null(string: *mut c_char) {
    if !string.is_null() {
        unsafe { CString::from_raw(string as *mut c_char) };
    }
}

extern "C" fn apply(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void) {
    println!("All started!");
}
