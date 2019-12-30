use warp::{self, path, Filter};

fn main() {
    let index = warp::any().and(warp::fs::file("index.html"));
    let app = warp::path("app").and(warp::fs::dir("app"));

    warp::serve(app.or(index))
        .run(([0, 0, 0, 0], 9980));
}
