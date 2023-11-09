use axum::{
    routing::{get, get_service},
    Router
};
use tower_http::services::{ServeFile, ServeDir};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get_service(ServeFile::new("static/index.html")))
        .route("/world", get(world))
        .route("/timeline", get_service(ServeFile::new("static/timeline.html")))
        .nest_service("/assets", get_service(ServeDir::new("static")));


    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



async fn world() -> &'static str {
    "               _-o#&&*''''?d:>b\\_\n          _o/\"`''  '',, dMF9MMMMMHo_\n       .o&#'        `\"MbHMMMMMMMMMMMHo.\n     .o\"\" '         vodM*$&&HMMMMMMMMMM?.\n    ,'              $M&ood,~'`(&##MMMMMMH\\\n   /               ,MMMMMMM#b?#bobMMMMHMMML\n  &              ?MMMMMMMMMMMMMMMMM7MMM$R*Hk\n ?$.            :MMMMMMMMMMMMMMMMMMM/HMMM|`*L\n|               |MMMMMMMMMMMMMMMMMMMMbMH'   T,\n$H#:            `*MMMMMMMMMMMMMMMMMMMMb#}'  `?\n]MMH#             \"\"*\"\"\"\"*#MMMMMMMMMMMMM'    -\nMMMMMb_                   |MMMMMMMMMMMP'     :\nHMMMMMMMHo                 `MMMMMMMMMT       .\n?MMMMMMMMP                  9MMMMMMMM}       -\n-?MMMMMMM                  |MMMMMMMMM?,d-    '\n :|MMMMMM-                 `MMMMMMMT .M|.   :\n  .9MMM[                    &MMMMM*' `'    .\n   :9MMk                    `MMM#\"        -\n     &M}                     `          .-\n      `&.                             .\n        `~,   .                     ./\n            . _                  .-\n              '`--._,dd###pp=\"\"'\n"
}