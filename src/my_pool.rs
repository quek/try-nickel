extern crate mysql;

use std::env;
use std::sync::Arc;
use std::path::PathBuf;
use mysql::{Pool, Opts};
use typemap::Key;
use nickel::{HttpRouter, Middleware, MiddlewareResult, Request, Response};
use plugin::{Pluggable, Extensible};


pub struct MyPoolMiddleware {
    pool: Arc<Pool>,
}

impl MyPoolMiddleware {
    pub fn new() -> MyPoolMiddleware {
        let url = match env::var("DB_URL") {
            Ok(val) => val,
            Err(_) => "mysql://root:@localhost:3307/outing_development".to_string(),
        };
        let pool = Pool::new(Opts::from_url(&url).unwrap()).unwrap();
        MyPoolMiddleware { pool: Arc::new(pool) }
    }
}

impl Key for MyPoolMiddleware { type Value = Arc<Pool>; }

impl<D> Middleware<D> for MyPoolMiddleware {
    fn invoke<'mw, 'conn>(&self,
                          req: &mut Request<'mw, 'conn, D>,
                          res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<MyPoolMiddleware>(self.pool.clone());
        res.next_middleware()
    }
}

pub trait MyPoolRequestExtensions {
    fn db(&self) -> &Pool;
}

impl<'a, 'b, D> MyPoolRequestExtensions for Request<'a, 'b, D> {
    fn db(&self) -> &Pool {
        let arc = self.extensions().get::<MyPoolMiddleware>().unwrap();
        &**arc
    }
}
