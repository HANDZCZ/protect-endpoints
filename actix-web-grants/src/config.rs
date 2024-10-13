use std::{any::Any, collections::HashSet, sync::Arc};

use actix_web::HttpResponse;

/// Grants error config for specific grants type configuration
///
/// ```
/// use actix_web_grants::GrantErrorConfig;
/// use actix_web::{App, HttpResponse};
///
/// let app = App::new().app_data(
///     // each grants type needs it's own config
///     GrantErrorConfig::<String>::default().error_handler(|_condition, _grants| {
///         HttpResponse::Forbidden().finish()
///     })
/// );
/// ```
#[derive(Clone, Default)]
pub struct GrantErrorConfig<T> {
    #[allow(clippy::type_complexity)]
    pub err_handler: Option<Arc<dyn Fn(&str, &HashSet<T>) -> HttpResponse + Send + Sync>>,
}

impl<T> GrantErrorConfig<T> {
    /// Set custom error handler.
    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, &HashSet<T>) -> HttpResponse + Send + Sync + 'static,
    {
        self.err_handler = Some(Arc::new(f));
        self
    }
}

/// Grants config for all grants types
///
/// ```
/// use actix_web_grants::GrantsConfig;
/// use actix_web::{App, HttpResponse};
///
/// let app = App::new().app_data(
///     GrantsConfig::default().default_error_handler(|_condition, _grants| {
///         HttpResponse::Forbidden().finish()
///     })
/// );
/// ```
#[derive(Clone, Default)]
pub struct GrantsConfig {
    #[allow(clippy::type_complexity)]
    pub err_handler: Option<Arc<dyn Fn(&str, &[&dyn Any]) -> HttpResponse + Send + Sync>>,
}

impl GrantsConfig {
    /// Set custom error handler.
    pub fn default_error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, &[&dyn Any]) -> HttpResponse + Send + Sync + 'static,
    {
        self.err_handler = Some(Arc::new(f));
        self
    }
}
