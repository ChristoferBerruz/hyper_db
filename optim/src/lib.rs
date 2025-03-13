use pgrx::prelude::*;
use serde::{Serialize, Deserialize};

::pgrx::pg_module_magic!();

#[derive(PostgresEnum, Serialize)]
pub enum OptimizationDirection {
    Maximize,
    Minimize
}

#[derive(Serialize, Deserialize, PostgresType)]
struct Hyperparameter {
    value: f64
}

#[derive(Serialize, Deserialize, PostgresType)]
struct Trial {
    hyperparameters: Vec<Hyperparameter>,
    score: f64
}

#[pg_extern]
fn hello_optim() -> &'static str {
    "Hello, optim"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_optim() {
        assert_eq!("Hello, optim", crate::hello_optim());
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
