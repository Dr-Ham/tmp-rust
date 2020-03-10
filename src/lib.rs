extern crate web_sys;
extern crate wasm_bindgen_futures;
extern crate console_error_panic_hook;
extern crate serde;
extern crate serde_derive;
extern crate reqwest;


use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub async fn get_player(id: i32) -> Result<(), JsValue> {
 // std::panic::set_hook(Box::new(console_error_panic_hook::hook)); -> Debug purposes


let url = format!("https://api.truckersmp.com/v2/player/{}", id);
let res = reqwest::Client::new().get(&url).send().await?;
let text = res.text().await?;


let vals: serde_json::Value = serde_json::from_str(&text).unwrap();

let rank = format!("{}", vals["response"]["groupName"]).replace("\"", "");
let name = format!("{name}", name = vals["response"]["name"]).replace("\"", "");



let rankhtml = match rank.as_str(){
     "Retired Legend Team Member"  | "Retired Team Member" => "<p style='color: #009688'>",
     "Developer" | "Project Manager" | "Project Coordinator" | "Senior Human Resource" | "Human Resource " | "Add-On Manager" | "Add-On Team" => "<p style='color: #673ab7'>",
     "Game Moderator" | "Game Manager" | "Game Moderator Leader" | "Report Moderator" | "Trainee" | "Senior Game Manager" => "<p style='color: #ff1744'>",
      _ => "<p style='color: #8bc34a'> ",
};

let finalhtml = format!("{rankhtm} {username} ", rankhtm = rankhtml, username = &name);

let window = web_sys::window().expect("UH OH");
let document = window.document().expect("Document not found");
let body = document.body().expect("Body not found");
let div_to_append = document.get_element_by_id("s").unwrap();

div_to_append.set_inner_html(&finalhtml);

div_to_append.append_child(&div_to_append)?;




Ok(())


}

#[wasm_bindgen]
pub async fn get_servers() -> Result<(), JsValue>{
    let res = reqwest::Client::new().get("https://api.truckersmp.com/v2/servers").send().await?;
    let text = res.text().await?;
    let vals: serde_json::Value = serde_json::from_str(&text).unwrap();


    for x in 0..9{
    let gameL = format!("{}", vals["response"][x]["game"]); 
    let max: f32 = format!("{}", vals["response"][x]["maxplayers"]).parse().unwrap();
    let actual: f32 = format!("{}",vals["response"][x]["players"]).parse().unwrap();

    let percent = format!("{}", (actual/max)*100.0);

    let finalhtml = format!("<div class='card' style='width: 350px; height: 350px;'>
      <div class='card-header'>
      <img src='https://stats.truckersmp.com/static/truckersmp/{game}.png' height='28' width='64'></img> {name}
       </div>  <div class='card-body'><h1>{players}</h1><small>/{maxplayers} ({percenta}) </small>
       <ul class='list-unstyled mt-3 mb-4'>
       <li>{queue} in queue</li>
       <li> Has speed limit? {speedlimit} </li>
       <li> Event: {event} </li>
       <li> Collisions: {collisions} </li>
       <li> Cars for players: {carsforplayers} </li>
       <li>Tick Rate: {ticks}</li>
       </div></div>",
                         name = vals["response"][x]["name"], queue = vals["response"][x]["queue"],
                         game = gameL.to_lowercase(), players = actual,
                         maxplayers = max, ticks = vals["response"][x]["syncdelay"],
                         percenta = percent, speedlimit = vals["response"][x]["speedlimiter"],
                         event = vals["response"][x]["event"], collisions = vals["response"][x]["collisions"],
                         carsforplayers = vals["response"][x]["carsforplayers"]
                        ).replace("\"", "");


                         
     let window = web_sys::window().expect("Window not found");
     let document = window.document().expect("Document not found");
     let body = document.body().expect("No body");
     let div_to_append = document.get_element_by_id("c").unwrap();    
     let new_div = document.create_element("div").unwrap();
    

       new_div.set_inner_html(&finalhtml);
       div_to_append.append_child(&new_div)?;

    }


    Ok(())

}


#[wasm_bindgen]
pub async fn get_game_info() -> Result<(), JsValue>{
  let res = reqwest::Client::new().get("https://api.truckersmp.com/v2/servers").send().await?;
  let text = res.text().await?;
  let vals: serde_json::Value = serde_json::from_str(&text).unwrap();

  //TODO


  Ok(())

}