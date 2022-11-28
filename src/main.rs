#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content::Html;


#[get("/")]
fn index() -> Html<&'static str> {
    let hello = "Hello, world!\n\nこんにちは\n\nSep/16/2020\n";

    Html(hello)
}

#[get("/page_a")]
fn page_a() -> &'static str {
    let page_a = "こんにちは";

    page_a
}


#[get("/page_b")]
fn page_b() -> &'static str {
    let page_b = "今晩は";

    page_b
}


fn main() {
    rocket::ignite().mount("/", routes![index,page_a,page_b]).launch();
}
