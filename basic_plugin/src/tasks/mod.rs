use patoka::core::env;

pub mod master;
pub mod subtask;

pub fn start() {
    if let Some(v) = env::get_opt_var("basic_plugin.enabled") {
        if v == "true" {
            master::start();
        }
    }
}
