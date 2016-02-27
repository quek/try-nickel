#[macro_use] extern crate nickel;
extern crate mustache;
extern crate nickel_mustache;   // https://github.com/Ryman/nickel-mustache
extern crate rustc_serialize;
extern crate plugin;
extern crate typemap;
extern crate mysql;

use nickel_mustache::Render;
use nickel::{HttpRouter, Nickel};

use mustache::MapBuilder;

use mysql::from_row;
use my_pool::MyPoolRequestExtensions;

mod my_pool;

#[derive(RustcEncodable)]
struct Region {
    id: i32,
    name: String,
}

fn main() {
    let mut server = Nickel::new();

    let my_pool_middleware = my_pool::MyPoolMiddleware::new();
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
