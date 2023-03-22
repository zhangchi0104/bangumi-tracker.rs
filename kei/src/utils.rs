use std::env::VarError;
#[macro_export]
macro_rules! derive_configure {
    ($scope:expr, $($component:expr),+) => {
        pub(super) fn configure(cfg: &mut actix_web::web::ServiceConfig) {
            cfg.service(actix_web::web::scope($scope)
                $(.service($component))+
        );
        }
    };
}
pub trait FromEnv
where
    Self: Sized,
{
    fn from_env() -> Result<Self, VarError>;
}

pub trait FromEnvDefault: FromEnv
where
    Self: Default,
{
    fn from_env_or_default() -> Self {
        Self::from_env().unwrap_or_default()
    }
}
