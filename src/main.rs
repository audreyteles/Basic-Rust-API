#[macro_use]
extern crate rocket;
extern crate object;

use rocket::serde::json::{json, Value};

/*
 Launch define que a função a seguir retornará uma
 instancia do servidor
 */
#[launch]
/*
 Quando tem o launch ele já deduz que o retorno será
 relacionado ao Rocket, então não precisa definir o tipo
 do retorno
*/
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/users/", routes![get_user, get_all_users])
}

/*
Pelo que eu entendi isso aqui é a definição da rota,
da mesma forma que fazemos no Python com FastAPI
@app.get("/")
def read_root():
    return {"Hello": "World"}
*/
#[get("/")] // Define a rota '/' como get
fn hello() -> Value { // Função que retorna uma string
    json!({"Hello":"world!"})
}

#[get("/<id>")]
fn get_user(id: u64) -> Value {
    json!({"id": id, "name": "Teste da Silva", "languages": ["Python", "JavaScript", "Rust"]})
}

#[get("/")]
fn get_all_users() -> Value {
    json!([{"id": 1, "name": "Teste da Silva", "languages": ["Python", "JavaScript", "Rust"]},{"id": 1, "name": "Bug dos Santos", "languages": ["Haskell", "GO"]}])
}
