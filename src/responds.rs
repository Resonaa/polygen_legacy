#[macro_export]
macro_rules! success {
    ($dat: expr) => {
        json!({"status": "success", "msg": $dat})
    };
}

#[macro_export]
macro_rules! error {
    ($dat: expr) => {
        json!({"status": "error", "msg": $dat})
    };
}
