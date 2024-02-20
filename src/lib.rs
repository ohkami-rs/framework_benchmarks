#![allow(non_snake_case)]

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

use ohkami::{FrontFang, BackFang, Request, Response, typed::ResponseBody};

pub struct SetServer;
impl BackFang for SetServer {
    type Error = std::convert::Infallible;
    #[inline] async fn bite(&self, res: &mut Response, _req: &Request) -> Result<(), Self::Error> {
        res.headers.set().Server("ohkami");
        Ok(())
    }
}

pub struct ConnectionPool<DB: sqlx::Database>(
    pub sqlx::Pool<DB>,
);
impl<DB: sqlx::Database> FrontFang for ConnectionPool<DB> {
    type Error = std::convert::Infallible;
    #[inline] async fn bite(&self, req: &mut Request) -> Result<(), Self::Error> {
        req.memorize(self.0.clone());
        Ok(())
    }
}

#[ResponseBody(JSONS)]
pub struct Message {
    pub message: &'static str,
}

#[derive(sqlx::FromRow)]
pub struct Fortune {
    pub id:      i32,
    pub message: String,
}

#[derive(sqlx::FromRow)]
#[ResponseBody(JSONS)]
pub struct World {
    pub id:           i32,
    pub randomNumber: i32,
}

pub struct MultipleWorlds(
    pub Vec<World>,
);
