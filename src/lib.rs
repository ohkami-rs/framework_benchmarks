#[macro_export]
/// ```
/// load_env! { ENV_1, ENV2 }
/// ```
macro_rules! load_env {
    ($( $name:ident : $t:ty ),* $(,)?) => {
        $(
            #[allow(non_snake_case)]
            pub fn $name() -> $t {
                static $name: ::std::sync::OnceLock<Result<String, ::std::env::VarError>> = ::std::sync::OnceLock::new();
            
                $name.get_or_init(|| ::std::env::var(::std::stringify!($name)))
                    .as_ref().unwrap()
                    .parse::<$t>()
                    .unwrap()
            }
        )*
    };
}

pub struct SetServer;
impl ohkami::BackFang for SetServer {
    #[inline] async fn bite(&self, res: &mut ohkami::Response, _req: &ohkami::Request) -> Result<(), ohkami::Response> {
        res.headers.set().Server("ohkami");
        Ok(())
    }
}

pub struct ConnectionPool<DB: sqlx::Database>(
    pub sqlx::Pool<DB>,
);
impl<DB: sqlx::Database> ohkami::FrontFang for ConnectionPool<DB> {
    #[inline] async fn bite(&self, req: &mut ohkami::Request) -> Result<(), ohkami::Response> {
        req.memorize(self.0.clone());
        Ok(())
    }
}
