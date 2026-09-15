#[macro_use]
extern crate rocket;

use app::build_rocket;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    build_rocket()
}
