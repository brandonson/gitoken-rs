extern crate hyper;
extern crate rustc_serialize as rcserialize;

use hyper::{Client, HttpError, HttpResult};
use hyper::client::Response;
use hyper::header::{Connection, ConnectionOption, Authorization, Basic};

use rcserialize::json;
use rcserialize::json::{BuilderError, Json, ToJson};

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use GitokenRequestError::*;
pub use scope::Scope;

pub mod scope;

#[derive(Debug)]
pub enum GitokenRequestError{
  GitokenHttpError(HttpError),
  GitokenUnexpectedStatusCode(Response),
  GitokenUnparseableContent(BuilderError),
  GitokenUnexpectedJson(Json),
}

impl From<HttpError> for GitokenRequestError {
  fn from(err: HttpError) -> GitokenRequestError {
    GitokenHttpError(err)
  }
}

impl From<BuilderError> for GitokenRequestError {
  fn from(err: BuilderError) -> GitokenRequestError {
    GitokenUnparseableContent(err)
  }
}

impl fmt::Display for GitokenRequestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.description())
  }
}

impl Error for GitokenRequestError{
  fn description(&self) -> &str {
    match *self {
      GitokenHttpError(_) => "HTTP request failed",
      GitokenUnexpectedStatusCode(_) => "Unexpected HTTP status code returned",
      GitokenUnparseableContent(_) => "HTTP response body contents could not be parsed",
      GitokenUnexpectedJson(_) => "Unexpected HTTP response json",
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      GitokenHttpError(ref err) => Some(err as &Error),
      GitokenUnparseableContent(ref err) => Some(err as &Error),
      _ => None,
    }
  }
}

pub struct GithubToken {
  pub token: String,
  pub url: String,
}

pub fn create_token(uname: &str, pass: &str, scopes:&[Scope]) -> Result<GithubToken, GitokenRequestError> {
  create_token_with_note(uname, pass, scopes, "Created by Gitoken")
}

pub fn create_token_with_note(
    uname: &str,
    pass: &str,
    scopes: &[Scope],
    note: &str) -> Result<GithubToken, GitokenRequestError> {

  let json = try!(fetch_token_json(uname, pass, scopes, note));

  if let Json::Object(json_map) = json {
    extract_token_from_json_map(json_map)
  } else {
    Err(GitokenUnexpectedJson(json))
  }
}

fn fetch_token_json(uname: &str,
                    password: &str,
                    scopes: &[Scope],
                    note: &str) -> Result<Json, GitokenRequestError> {
  let request_json = build_token_creation_json(scopes, note);

  let mut client = Client::new();
  let req_json_str = request_json.to_string();
  let request = client.post("https://api.github.com/authorizations")
                      .header(Connection(vec![ConnectionOption::Close]))
                      .header(Authorization(Basic{username: uname.to_string(),
                                                  password: Some(password.to_string())}))
                      .body(AsRef::<str>::as_ref(&req_json_str));

  let mut result = try!(request.send());
  Ok(try!(Json::from_reader(&mut result)))
}

fn build_token_creation_json(scopes: &[Scope], note:&str) -> Json {
  let scope_string_list:Vec<String> = scopes.iter().map(|scope| scope.token_scope_string()).collect();

  let mut json_map:HashMap<String, Json> = HashMap::new();
  json_map.insert("scopes".to_string(), scope_string_list.to_json());
  json_map.insert("note".to_string(), note.to_json());

  json_map.to_json()
}

fn extract_token_from_json_map(map: json::Object) -> Result<GithubToken, GitokenRequestError> {
  //need this to avoid having map borrowed in the error case
  {
    let json_value_tuple = (map.get("token"), map.get("url"));

    //If we got both, and have the right json value type, use it for the token
    if let (Some(&Json::String(ref token_value)), Some(&Json::String(ref url_value)))
            = json_value_tuple{
      return Ok(GithubToken{token: token_value.to_string(), url: url_value.to_string()})
    }
  }
  Err(GitokenUnexpectedJson(Json::Object(map)))
}

pub fn delete_token(uname: &str, password: &str, token:&GithubToken) -> HttpResult<Response> {
  delete_token_by_url(uname, password, AsRef::<str>::as_ref(&token.url))
}

pub fn delete_token_by_url(uname: &str, password: &str, url:&str) -> HttpResult<Response> {
  let mut client = Client::new();

  let request = client.delete(url)
                      .header(Connection(vec![ConnectionOption::Close]))
                      .header(Authorization(Basic{username: uname.to_string(),
                                                  password: Some(password.to_string())}));
  request.send()
}

