use tracing::{error, info, warn};

struct File {
    path: String,
    body: String,
}

impl File {
    fn new(path: String) -> Self {
        Self {
            path,
            body: String::new(),
        }
    }
}
