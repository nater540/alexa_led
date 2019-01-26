use serde_derive::{Deserialize};

use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Request {
  // The version specifier for the request with the value defined as: "1.0"
  pub version: String,

  // The session object provides additional context associated with the request.
  pub session: Option<Session>,

  // The context object provides your skill with information about the current state of the Alexa service and device at the time the request is sent to your service.
  pub context: Context,

  // A request object that provides the details of the user's request. There are several different request types available.
  pub request: RequestBody
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Session {
  // A boolean value indicating whether this is a new session. Returns true for a new session or false for an existing session.
  new: bool,

  // A string that represents a unique identifier per a user's active session.
  session_id: String,

  // A map of key-value pairs. The attributes map is empty for requests where a new session has started with the property new set to true.
  attributes: Option<BTreeMap<String, serde_json::Value>>,

  // An object containing an application ID. This is used to verify that the request was intended for your service.
  application: Application,

  // An object that describes the user making the request.
  user: User
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Context {
  #[serde(rename = "System")]
  pub system: SystemContext,

  #[serde(rename = "AudioPlayer")]
  pub audio_player: AudioPlayerContext
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemContext {
  // A string containing a token that can be used to access Alexa-specific APIs.
  pub api_access_token: String,

  // A string that references the correct base URI to refer to by region, for use with APIs such as the Device Location API and Progressive Response API.
  pub api_endpoint: String,

  // An object containing an application ID. This is used to verify that the request was intended for your service.
  pub application: Application,

  // An object providing information about the device used to send the request.
  pub device: Device,

  // An object that describes the user making the request.
  pub user: User
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioPlayerContext {

}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
  pub device_id: String,
  pub supported_interfaces: BTreeMap<String, serde_json::Value>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
  // Describes the request type with the value as: "LaunchRequest"
  #[serde(rename = "type")]
  pub request_type: RequestType,

  // 	Provides the date and time when Alexa sent the request as an ISO 8601 formatted string.
  pub timestamp: String,

  // Represents a unique identifier for the specific request.
  pub request_id: String,

  // 	A string indicating the user's locale. For example: "en-US".
  pub locale: String,

  #[serde(flatten)]
  pub intent: Option<IntentRequest>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RequestType {
  LaunchRequest,
  IntentRequest,
  SessionEndedRequest
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntentRequest {
  pub dialog_state: String,
  pub intent: Intent
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Intent {
  pub name: String,
  pub confirmation_status: String,
  pub slots: BTreeMap<String, Slot>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
  pub name: String,
  pub value: String,
  pub confirmation_status: String,
  pub resolutions: SlotResolution
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SlotResolution {
  pub resolutions_per_authority: Vec<serde_json::Value>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SlotResolutionAuthority {
  // The name of the authority for the slot values. For custom slot types, this authority label incorporates your skill ID and the slot type name.
  pub authority: String,

  // An object representing the status of entity resolution for the slot.
  pub status: SlotResolutionAuthorityStatus
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SlotResolutionAuthorityStatus {
  /*
   * A code indicating the results of attempting to resolve the user utterance against the defined slot types.
   * This can be one of the following:
   * - ER_SUCCESS_MATCH: The spoken value matched a value or synonym explicitly defined in your custom slot type.
   * - ER_SUCCESS_NO_MATCH: The spoken value did not match any values or synonyms explicitly defined in your custom slot type.
   * - ER_ERROR_TIMEOUT: An error occurred due to a timeout.
   * - ER_ERROR_EXCEPTION: An error occurred due to an exception during processing.
  */
  pub code: String,
  pub values: Vec<serde_json::Value>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Application {
  pub application_id: String
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub user_id: String,
  pub access_token: Option<String>,
  pub permissions: Option<BTreeMap<String, serde_json::Value>>
}


#[cfg(test)]
mod tests {
  use indoc::indoc;
  use super::*;

  #[test]
  fn deserializes_intent_request() -> Result<(), String> {
    let test_json = indoc!(r#"
      {
        "version": "1.0",
        "session": {
          "new": true,
          "sessionId": "amzn1.echo-api.session.[unique-value-here]",
          "application": {
            "applicationId": "amzn1.ask.skill.[unique-value-here]"
          },
          "attributes": {
            "key": "string value"
          },
          "user": {
            "userId": "amzn1.ask.account.[unique-value-here]",
            "accessToken": "Atza|AAAAAAAA...",
            "permissions": {
              "consentToken": "ZZZZZZZ..."
            }
          }
        },
        "context": {
          "System": {
            "device": {
              "deviceId": "string",
              "supportedInterfaces": {
                "AudioPlayer": {}
              }
            },
            "application": {
              "applicationId": "amzn1.ask.skill.[unique-value-here]"
            },
            "user": {
              "userId": "amzn1.ask.account.[unique-value-here]",
              "accessToken": "Atza|AAAAAAAA...",
              "permissions": {
                "consentToken": "ZZZZZZZ..."
              }
            },
            "apiEndpoint": "https://api.amazonalexa.com",
            "apiAccessToken": "AxThk..."
          },
          "AudioPlayer": {
            "playerActivity": "PLAYING",
            "token": "audioplayer-token",
            "offsetInMilliseconds": 0
          }
        },
        "request": {
          "type": "IntentRequest",
          "requestId": "string",
          "timestamp": "string",
          "dialogState": "string",
          "locale": "string",
          "intent": {
            "name": "string",
            "confirmationStatus": "string",
            "slots": {
              "SlotName": {
                "name": "string",
                "value": "string",
                "confirmationStatus": "string",
                "resolutions": {
                  "resolutionsPerAuthority": [
                    {
                      "authority": "string",
                      "status": {
                        "code": "string"
                      },
                      "values": [
                        {
                          "value": {
                            "name": "string",
                            "id": "string"
                          }
                        }
                      ]
                    }
                  ]
                }
              }
            }
          }
        }
      }"#);

    match serde_json::from_str::<Request>(&test_json) {
      Result::Err(err) => { Err(format!("Failed to deserialize test string ({})", err)) },
      Result::Ok(request) => {
        match request.session {
          Some(session) => {
            if session.new == true {
              // Great Success!
              Ok(())
            }
            else {
              Err(format!("Expected session.new == `true`, got `{}`", session.new))
            }
          },
          None => { Err(format!("Request session object missing/invalid")) }
        }
      }
    }
  }
}
