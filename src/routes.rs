use crate::cubes::{CubeColor, Cubes};
use actix_web::web::{Data, Json};
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::slice::SliceIndex;
use std::sync::Mutex;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CubeRequestInfo {
    pub x: usize,
    pub y: usize,
    pub cube_color: String,
}

impl Default for CubeRequestInfo {
    fn default() -> Self {
        CubeRequestInfo {
            x: 0,
            y: 0,
            cube_color: "".to_string(),
        }
    }
}

/// Return an HttpResponse 200
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

//
pub async fn fetch_cubes(req: HttpRequest) -> HttpResponse {
    //let data = req.app_data::<Data<Mutex<MyData>>>().unwrap();
    //let x = req.app_data::<Data<Cubes>>().unwrap();
    //let mut my_x = x.grid.lock().unwrap();
    //let z = *my_x;

    match serde_json::to_string(&*req.app_data::<Data<Cubes>>().unwrap().grid.read().unwrap()) {
        Ok(serialized) => HttpResponse::Ok().body(serialized),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn set_cube(req: HttpRequest, info: Json<CubeRequestInfo>) -> HttpResponse {
    let x = info.x;
    let y = info.y;
    let desired_cube_color = info.cube_color.clone();

    let actix_data = req.app_data::<Data<Cubes>>().unwrap();
    let mut my_data = actix_data.grid.write().unwrap();
    let mut worker = my_data.clone();

    if desired_cube_color == String::from("Blue") {
        worker[x][y] = CubeColor::Blue;
    } else if desired_cube_color == String::from("Green") {
        worker[x][y] = CubeColor::Green;
    } else if desired_cube_color == String::from("Red") {
        worker[x][y] = CubeColor::Red;
    } else if desired_cube_color == String::from("White") {
        worker[x][y] = CubeColor::White;
    } else if desired_cube_color == String::from("Yellow") {
        worker[x][y] = CubeColor::Yellow;
    } else if desired_cube_color == String::from("Black") {
        worker[x][y] = CubeColor::Black;
    } else if desired_cube_color == String::from("Purple") {
        worker[x][y] = CubeColor::Purple;
    } else if desired_cube_color == String::from("Orange") {
        worker[x][y] = CubeColor::Orange;
    } else {
        return HttpResponse::InternalServerError().finish();
    };

    *my_data = worker;

    HttpResponse::Ok().finish()
}

/*pub async fn set_cube_color(
    rw_cube_data: Data<Cubes>,
    cube_request_info: web::Json<CubeRequestInfo>,
) -> HttpResponse {
    let x = cube_request_info.x;
    let y = cube_request_info.y;
    let desired_cube_color = cube_request_info.cube_color.clone();
    let mut lock = rw_cube_data.lock();
    //let mut current_cube_color = lock.grid.get(x).unwrap().get(y).unwrap();

    *lock.grid.get(0).unwrap().get(0).unwrap() = CubeColor::Orange;
    if desired_cube_color == String::from("Blue") {
        //current_cube_color = CubeColor::Blue;
    } else if desired_cube_color == String::from("Green") {
        //current_cube_color = CubeColor::Green;
    } else if desired_cube_color == String::from("Red") {
        //current_cube_color = CubeColor::Red;
    } else if desired_cube_color == String::from("White") {
        //current_cube_color = CubeColor::White;
    } else if desired_cube_color == String::from("Yellow") {
        //current_cube_color = CubeColor::Yellow;
    } else if desired_cube_color == String::from("Black") {
        //current_cube_color = CubeColor::Black;
    } else if desired_cube_color == String::from("Purple") {
        //current_cube_color = CubeColor::Purple;
    } else if desired_cube_color == String::from("Orange") {
        //current_cube_color = CubeColor::Orange;
    } else {
        return HttpResponse::InternalServerError().finish();
    };

    //eprintln!("current_cube_color = {:#?}", current_cube_color);

    HttpResponse::Ok().finish()
}*/
