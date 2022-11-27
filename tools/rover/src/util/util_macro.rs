#[macro_export]
macro_rules! launch_git {
    ($workdir: expr, $($arg: expr),+) => {{
        let mut cmd = process::Command::new("git");
        cmd.current_dir($workdir);
        $(cmd.arg($arg);)*
        cmd.status()?.success()
    }};
}
