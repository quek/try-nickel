#[macro_use] extern crate nickel;
extern crate mysql;
extern crate mustache;
extern crate nickel_mustache;   // https://github.com/Ryman/nickel-mustache
extern crate rustc_serialize;
extern crate plugin;
extern crate typemap;

use std::sync::Arc;

use nickel_mustache::Render;
use nickel::{HttpRouter, Middleware, MiddlewareResult, Nickel, Request, Response};

use typemap::Key;
use plugin::{Pluggable, Extensible};

use mustache::MapBuilder;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;

struct MyPoolMiddleware {
    pool: Arc<MyPool>,
}

impl MyPoolMiddleware {
    fn new() -> MyPoolMiddleware {
        let opts = MyOpts {
            user: Some("root".to_string()),
            pass: Some("".to_string()),
            db_name: Some("outing_r3_development".to_string()),
            ..Default::default()
        };
        let pool = MyPool::new(opts).unwrap();
        MyPoolMiddleware { pool: Arc::new(pool) }
    }
}

impl Key for MyPoolMiddleware { type Value = Arc<MyPool>; }

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
    fn db(&self) -> &MyPool;
}

impl<'a, 'b, D> MyPoolRequestExtensions for Request<'a, 'b, D> {
    fn db(&self) -> &MyPool {
        let arc = self.extensions().get::<MyPoolMiddleware>().unwrap();
        &**arc
    }
}


#[derive(RustcEncodable)]
struct Region {
    id: i32,
    name: String,
}

fn main() {
    let mut server = Nickel::new();

    let my_pool_middleware = MyPoolMiddleware::new();
    server.utilize(my_pool_middleware);

    server.get("/", middleware!{ |request, response|

        let mut data = MapBuilder::new();
        data = data.insert_str("title", "ちーまいか");
        data = data.insert_str("subject", "もふもふ");

        data = data.insert_vec("regions", |builder| {
            let mut builder = builder;
            let result = request.db().prep_exec("select id, name from regions order by rand()", ()).unwrap();
            for row in result {
                let row = row.unwrap();
                let (id, name) = from_row::<(i32, String)>(row);
                builder = builder.push(&Region { id: id, name: name }).ok().unwrap();
            }
            builder
        });


        let regions: Vec<Region> =
            request.db().prep_exec("select id, name from regions order by rand()", ()).unwrap().map(|row| {
                let (id, name) = from_row::<(i32, String)>(row.unwrap());
                Region { id: id, name: name }
            }).collect();
        data = data.insert("region-list", &regions).ok().unwrap();

        return response.render_data_with_layout("assets/main",
                                                "assets/layout",
                                                &data.build());
    });

    server.listen("127.0.0.1:9000");
}
