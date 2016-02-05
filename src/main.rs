#[macro_use] extern crate nickel;
extern crate mysql;
extern crate mustache;
extern crate nickel_mustache;   // https://github.com/Ryman/nickel-mustache
extern crate rustc_serialize;

use nickel_mustache::Render;
use nickel::{Nickel, HttpRouter};

use mustache::MapBuilder;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;

// cargo test -- --nocapture
#[test]
fn test_mysql() {
    let opts = MyOpts {
        user: Some("root".to_string()),
        pass: Some("".to_string()),
        db_name: Some("outing_r3_development".to_string()),
        ..Default::default()
    };
    let pool = MyPool::new(opts).unwrap();

    let result = pool.prep_exec("select id, name from regions limit 1", ()).unwrap();
    for row in result {
        let row = row.unwrap();
        println!("{:?}, {:?}", row[0], row[1]);
        let (id, name) = from_row::<(i32, String)>(row);
        println!("{:?}, {:?}", id, name);
    }
}

#[derive(RustcEncodable)]
struct Region {
    id: i32,
    name: String,
}

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! {
        |_request, response|

        let mut data = MapBuilder::new();
        data = data.insert_str("title", "ちーまいか");
        data = data.insert_str("subject", "もふもふ");

        let opts = MyOpts {
            user: Some("root".to_string()),
            pass: Some("".to_string()),
            db_name: Some("outing_r3_development".to_string()),
            ..Default::default()
        };
        let pool = MyPool::new(opts).unwrap();

        let result = pool.prep_exec("select id, name from regions", ()).unwrap();
        for row in result {
            let row = row.unwrap();
            println!("{:?}, {:?}", row[0], row[1]);
            let (id, name) = from_row::<(i32, String)>(row);
            println!("{:?}, {:?}", id, name);
            data = data.insert_map("region", |builder| {
                builder.insert("id", &id).ok().unwrap().
                    insert("name", &name).ok().unwrap()
            });
            data = data.insert("region", &Region { id: id, name: name }).ok().unwrap()
        }

        return response.render_data_with_layout("assets/main",
                                                "assets/layout",
                                                &data.build());
    });

    server.listen("127.0.0.1:9000");
}
