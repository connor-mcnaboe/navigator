use actix_web::{Result, web, Error, post, Responder, HttpResponse};
use tareldar::mission::{Mission};
use tareldar::orbit::{Orbit, CoordinateSystem, KeplerElements};
use tareldar::propagator::{OdeSolver, propagate};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use tareldar::bodies::CentralBody;

#[post("/")]
pub async fn mission(params: web::Json<MissionDto>) -> impl Responder {
    let mission = mission_from_dto(params.into_inner());
    let result= propagate(mission);
    // TODO: Fix return type in tareldar to be easier on callers, not passthrough from ode_solvers lib:
    let mut formatted_result = vec![];
    for vec in result {
        formatted_result.push(
            StateVector {
                position: (vec[0], vec[1], vec[2]),
                velocity: (vec[3], vec[4], vec[5])
            }
        )
    }
    HttpResponse::Ok().json(formatted_result)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateVector {
    pub position: (f64, f64, f64),
    pub velocity: (f64, f64, f64)
}

#[derive(Serialize, Deserialize)]
pub struct MissionDto {
    pub orbit: OrbitDto,
    pub epoch: f64,
    pub duration: f64,
}

pub fn mission_from_dto(dto: MissionDto) -> Mission {
    Mission {
        orbit: orbit_from_dto(dto.orbit),
        epoch: dto.epoch,
        duration: dto.duration
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrbitDto {
    pub kepler_elements: KeplerElementsDto,
    pub central_body: String,
    pub coordinate_system: String,
    pub ode_solver: String,
}

pub fn orbit_from_dto(dto: OrbitDto) -> Orbit {
    Orbit {
        kepler_elements: kepler_elements_from_dto(dto.kepler_elements),
        ode_solver: OdeSolver::from_str(&dto.ode_solver).unwrap(),
        coordinate_system: CoordinateSystem::from_str(&dto.coordinate_system).unwrap(),
        central_body: CentralBody::from_str(&dto.central_body).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeplerElementsDto {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub longitude_of_ascending_node: f64,
    pub argument_of_periapsis: f64,
    pub true_anomaly: f64,
}

pub fn dto_from_kepler_elements(kepler_elements: KeplerElements) -> KeplerElementsDto {
    KeplerElementsDto {
            semi_major_axis: kepler_elements.semi_major_axis,
            eccentricity: kepler_elements.eccentricity,
            inclination: kepler_elements.inclination,
            longitude_of_ascending_node: kepler_elements.longitude_of_ascending_node,
            argument_of_periapsis: kepler_elements.argument_of_periapsis,
            true_anomaly: kepler_elements.true_anomaly,
    }
}

pub fn kepler_elements_from_dto(dto: KeplerElementsDto) -> KeplerElements {
    KeplerElements {
        semi_major_axis: dto.semi_major_axis,
        eccentricity: dto.eccentricity,
        inclination: dto.inclination,
        longitude_of_ascending_node: dto.longitude_of_ascending_node,
        argument_of_periapsis: dto.argument_of_periapsis,
        true_anomaly: dto.true_anomaly,
    }
}

