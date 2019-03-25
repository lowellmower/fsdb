#[derive(Debug)]
pub struct Bucket {
    id: String
}

impl Bucket {
    pub fn new(id: String) -> Self {
        Bucket { id: id }
    }
}
