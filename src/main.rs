use fsdb::db::*;

fn main() {
    let b = Bucket::new("bucket_uid".to_string());
    println!("{:?}", b);
}
