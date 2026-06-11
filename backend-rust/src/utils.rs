use std::env;

pub fn is_local_runtime() -> bool {
    let app_env = env::var("APP_ENV")
        .or_else(|_| env::var("RUST_ENV"))
        .unwrap_or_default()
        .to_lowercase();

    matches!(app_env.as_str(), "local" | "development" | "dev")
        || (cfg!(debug_assertions) && app_env != "production")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_is_local_runtime_local() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("APP_ENV", "local");
            env::remove_var("RUST_ENV");
        }
        assert!(is_local_runtime());
    }

    #[test]
    fn test_is_local_runtime_dev() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("APP_ENV", "dev");
            env::remove_var("RUST_ENV");
        }
        assert!(is_local_runtime());
    }

    #[test]
    fn test_is_local_runtime_production() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("APP_ENV", "production");
            env::remove_var("RUST_ENV");
        }
        assert!(!is_local_runtime());
    }

    #[test]
    fn test_is_local_runtime_fallback_rust_env() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("APP_ENV");
            env::set_var("RUST_ENV", "development");
        }
        assert!(is_local_runtime());
    }

    #[test]
    fn test_is_local_runtime_empty() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("APP_ENV");
            env::remove_var("RUST_ENV");
        }
        // Should be true if running with debug_assertions, false otherwise
        let expected = cfg!(debug_assertions);
        assert_eq!(is_local_runtime(), expected);
    }
}

