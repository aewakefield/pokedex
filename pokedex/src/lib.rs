use pokeapi::{Pokeapi, PokeapiError, PokemonInfo};
use translator::{Translator, TranslatorError};

use core::future::Future;
use serde::Serialize;
use std::net::SocketAddr;
use warp::{http::StatusCode, reply::Response, Filter, Rejection, Reply};

pub fn run(
    addr: impl Into<SocketAddr> + 'static,
    pokeapi_url: &str,
    translator_url: &str,
) -> (SocketAddr, impl Future<Output = ()> + 'static) {
    let pokeapi = Pokeapi::new(pokeapi_url).expect("Invalid pokeapi url supplied");
    let translator = Translator::new(translator_url).expect("Invalid translator url supplied");

    let pokemon = warp::path!("pokemon" / String)
        .and(warp::get())
        .and(with_pokeapi(pokeapi.clone()))
        .and_then(info)
        .with(warp::trace::named("pokemon"));

    let translated = warp::path!("pokemon" / "translated" / String)
        .and(warp::get())
        .and(with_pokeapi(pokeapi))
        .and(with_translator(translator))
        .and_then(translated)
        .with(warp::trace::named("pokemon_translated"));

    let routes = pokemon
        .or(translated)
        .with(warp::trace::request())
        .recover(handle_rejection);

    warp::serve(routes).bind_ephemeral(addr)
}

#[tracing::instrument]
async fn info(pokemon_name: String, pokeapi: Pokeapi) -> Result<Response, Rejection> {
    let info = pokeapi.info(&pokemon_name).await;

    match info {
        Ok(Some(info)) => Ok(warp::reply::json(&info).into_response()),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(err) => Err(warp::reject::custom(PokeapiRejection::from(err))),
    }
}

#[tracing::instrument]
async fn translated(
    pokemon_name: String,
    pokeapi: Pokeapi,
    translator: Translator,
) -> Result<Response, Rejection> {
    let info = pokeapi.info(&pokemon_name).await;

    let info = match info {
        Ok(Some(info)) => info,
        Ok(None) => return Ok(StatusCode::NOT_FOUND.into_response()),
        Err(err) => return Err(warp::reject::custom(PokeapiRejection::from(err))),
    };

    let translated = translate_info(info, &translator).await;

    match translated {
        Ok(translated) => Ok(warp::reply::json(&translated).into_response()),
        Err(err) => Err(warp::reject::custom(TranslatorRejection::from(err))),
    }
}

async fn translate_info(
    mut info: PokemonInfo,
    translator: &Translator,
) -> Result<PokemonInfo, TranslatorError> {
    if let Some(description) = info.description {
        if info.habitat == "cave" || info.is_legendary {
            info.description = Some(translator.yoda(description).await?);
        } else {
            info.description = Some(translator.shakespeare(description).await?);
        }
    }

    Ok(info)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(pokeapi_rejection) = err.find::<PokeapiRejection>() {
        let json = warp::reply::json(&ErrorMessage {
            message: pokeapi_rejection.0.to_string(),
        });
        Ok(warp::reply::with_status(
            json,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(translator_rejection) = err.find::<TranslatorRejection>() {
        let json = warp::reply::json(&ErrorMessage {
            message: translator_rejection.0.to_string(),
        });
        Ok(warp::reply::with_status(
            json,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Err(err)
    }
}

fn with_pokeapi(
    pokeapi: Pokeapi,
) -> impl Filter<Extract = (Pokeapi,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pokeapi.clone())
}

fn with_translator(
    translator: Translator,
) -> impl Filter<Extract = (Translator,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || translator.clone())
}

#[derive(Debug)]
struct PokeapiRejection(PokeapiError);
impl warp::reject::Reject for PokeapiRejection {}
impl From<PokeapiError> for PokeapiRejection {
    fn from(err: PokeapiError) -> Self {
        Self(err)
    }
}

#[derive(Debug)]
struct TranslatorRejection(TranslatorError);
impl warp::reject::Reject for TranslatorRejection {}
impl From<TranslatorError> for TranslatorRejection {
    fn from(err: TranslatorError) -> Self {
        Self(err)
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}
