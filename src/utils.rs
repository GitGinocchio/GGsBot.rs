use worker::{Date, Env, Request, console_log};
use cfg_if::cfg_if;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn is_dev(env: &Env) -> bool {
    match env.var("WORKER_ENV") {
        Ok(v) => v.to_string() == "dev",
        Err(_) => false, // fallback a production se non definito
    }
}

pub fn log_request(req: &Request) {
    let cf = req.cf();

    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        cf.and_then(|cf| cf.coordinates()).unwrap_or_default(),
        cf.and_then(|cf| cf.region()).unwrap_or("unknown region".into())
    );
}
