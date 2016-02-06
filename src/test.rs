
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
