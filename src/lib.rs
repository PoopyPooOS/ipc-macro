#[macro_export]
macro_rules! define_commands {
    ($($command:ident($($arg:ty),*) -> $ret:ty ;)*) => {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Command {
            $($command($($arg),*)),*
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub enum Response {
            $(
                $command(
                    define_commands!(@return_type $ret)
                )
            ),*
        }
    };

    (@return_type ()) => {};

    (@return_type $ret:ty) => {
        $ret
    };
}
