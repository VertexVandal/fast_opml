use opml::OPML;
use rustler::{Env, Encoder, NifResult, Term};
use serde_json::json;

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse_opml(env: Env, opml_string: String) -> NifResult<Term> {
    let document = OPML::from_str(opml_string.as_str()).map_err(|err| format!("Unable to parse opml - ({:?})", err));

    let ser = serde_rustler::Serializer::from(env);
    let de = json!(document);
    let encoded = serde_transcode::transcode(de, ser).map_err(|_err| "Unable to encode to erlang terms");

    match encoded {
        Ok(term) => Ok(term),
        Err(error_message) => Ok((atoms::error(), error_message).encode(env)),
    }
}

rustler::init!("Elixir.FastOpml", [parse_opml]);
