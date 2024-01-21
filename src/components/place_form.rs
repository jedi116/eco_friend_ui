use reqwasm::http::{Request, Method};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde_json::Value;
use gloo::console::log;
use web_sys;

use crate::components::text_input::*;
use crate::components::button::*;
use crate::components::notification::Notification;
use serde::{Deserialize, Serialize};




#[derive(Debug, Deserialize, Serialize, Clone)]
struct PlacePrediction {
    description: String,
    place_id: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PlaceSearchResponse {
    predictions: Vec<PlacePrediction>
}


#[derive(Debug, Deserialize, Serialize)]
struct PlaceSearchRequest {
    place: String
}

struct FormValidationMessages {
    origin_field_error: String,
    destination_field_error: String
}

#[function_component]
pub fn PlaceForm() -> Html {
    let form_validation: UseStateHandle<FormValidationMessages> = use_state(|| FormValidationMessages {
        origin_field_error: "".to_string(),
        destination_field_error: "".to_string()
    });
    let form_validation_clone = form_validation.clone();
    let form_validation_clone2 = form_validation.clone();
    let form_validation_value = &*form_validation_clone;

    let origin_prediction_state: UseStateHandle<Option<PlaceSearchResponse>>  = use_state(|| None);
    let origin_prediction_state_clone = origin_prediction_state.clone();

    let destination_prediction_state: UseStateHandle<Option<PlaceSearchResponse>>  = use_state(|| None);
    let destination_prediction_state_clone = destination_prediction_state.clone();


    let destination_state = use_state(||"".to_owned());
    let copy_destination_state = destination_state.clone();

    let selected_destination_state = use_state(||"".to_owned());
    let selected_copy_destination_state = selected_destination_state.clone();

    let selected_destination_state_for_report = selected_destination_state.clone();

    let origin_state = use_state(||"".to_owned());
    let copy_origin_state = origin_state.clone();

    let selected_origin_state = use_state(||"".to_owned());
    let selected_copy_origin_state = selected_origin_state.clone();

    let selected_origin_state_for_report = selected_origin_state.clone();

    let handle_destination_input = Callback::from(move |value: String| {
        destination_state.set(value);
    });

    let handle_origin_input = Callback::from(move |value: String| {
        origin_state.set(value);
    });

    let handle_click_origin_search = Callback::from( move |value: String| {
        let clone_origin = copy_origin_state.clone();
        let clone_origin_prediction_state = origin_prediction_state.clone();
        let clone_form_validation = form_validation.clone();
        spawn_local(async move {
            let origin = &*clone_origin;
            let request_json = serde_json::to_value(PlaceSearchRequest {
                place: origin.to_string()
               }).unwrap().to_string();
               log!(JsValue::from(&request_json));
           if origin.len() > 2 {
            let response: PlaceSearchResponse = reqwasm::http::Request::post("http://localhost:8080/getPlacePredictions")
           .header("Content-Type", "application/json")
           .body( JsValue::from(request_json))
           .send()
           .await
           .unwrap()
           .json()
           .await
           .unwrap();
            let test = serde_json::to_value(response.clone()).unwrap();
            log!(JsValue::from_serde(&test).unwrap());
           clone_origin_prediction_state.set(Some(response));
           clone_form_validation.set(FormValidationMessages { origin_field_error: "".to_string(), destination_field_error: "".to_string() })
           } else {
            clone_form_validation.set(FormValidationMessages { origin_field_error: "need to input more than 2 character".to_string(), destination_field_error: "".to_string() })
           }

        })
    });

    let handle_click_destination_search = Callback::from(move |value: String| {
        let clone_destination = copy_destination_state.clone();
        let clone_destination_prediction_state = destination_prediction_state.clone();
        let clone_form_validation = form_validation_clone2.clone();
        spawn_local(async move {
            let destination = &*clone_destination;
            let request_json = serde_json::to_value(PlaceSearchRequest {
                place: destination.to_string()
               }).unwrap().to_string();
               log!(JsValue::from(&request_json));
           if destination.len() > 2 {
            let response: PlaceSearchResponse = reqwasm::http::Request::post("http://localhost:8080/getPlacePredictions")
           .header("Content-Type", "application/json")
           .body( JsValue::from(request_json))
           .send()
           .await
           .unwrap()
           .json()
           .await
           .unwrap();
            let test = serde_json::to_value(response.clone()).unwrap();
            log!(JsValue::from_serde(&test).unwrap());
            clone_destination_prediction_state.set(Some(response));
            clone_form_validation.set(FormValidationMessages { origin_field_error: "".to_string(), destination_field_error: "".to_string() })
           } else {
            clone_form_validation.set(FormValidationMessages { origin_field_error: "".to_string(), destination_field_error: " need to input more than 2 character".to_string() })
           }
        })
    });

    let handle_origin_prediction_click =  Callback::from(move |value: String| {
        selected_origin_state.set(value.clone());
    });
    let handle_destination_prediction_click =  Callback::from(move |value: String| {
        selected_destination_state.set(value.clone());
    });
    let handle_calculate_click =  Callback::from(move |value: String| {
        let redirect_uri = format!("/report/{}/{}",&*selected_origin_state_for_report, &*selected_destination_state_for_report);
        let window = web_sys::window().expect("can not load window");
        window
        .location()
                    .set_href(&redirect_uri)
                    .expect("failed to set the href");
    });
    html! {
        <div class="eco-form1">
            <div class="form">
                {
                    if form_validation_value.origin_field_error.len() > 1 {
                        html! {
                            <Notification not_type={"error".to_string()} message={form_validation_value.origin_field_error.clone()}/>
                        }
                    } else {
                        html! {
                            <span></span>
                        }
                    }
                }
                <TextInput name={"Origin"} handle_input={handle_origin_input} />
                <Button name={"Search origin"} handle_click= {handle_click_origin_search} class_style={"submit".to_string()}/>
                <div>
                    {
                    match &*origin_prediction_state_clone {
                        Some(values) => {
                            values.predictions.iter().map(|prediction| {
                                html! {
                                    <div class="place-suggestion-container">
                                        <Button 
                                            name={prediction.description.clone()} 
                                            handle_click= {handle_origin_prediction_click.clone()} 
                                            class_style={
                                                if prediction.description.clone().eq(&*selected_copy_origin_state) {"place-option-button-selected".to_string()}
                                                else {"place-option-button".to_string()}
                                                }
                                        />
                                    </div>
                                }
                            }).collect()
                        }
                        None => {
                            html! {
                                <h6 class="subtitle">{"No locations yet"}</h6>
                            } 
                        }
                    }
                }
                </div>
                
                <h5 class="subtitle">{"Selected origin:  "}{&*selected_copy_origin_state}</h5>
                {
                    if form_validation_value.destination_field_error.len() > 1 {
                        html! {
                            <Notification not_type={"error".to_string()} message={form_validation_value.destination_field_error.clone()}/>
                        }
                    } else {
                        html! {
                            <span></span>
                        }
                    }
                }
                <TextInput name={"Destination"} handle_input={handle_destination_input} />
                <Button name={"Search destination"} handle_click= {handle_click_destination_search} class_style={"submit".to_string()}/>
                <div>
                    {
                    match &*destination_prediction_state_clone {
                        Some(values) => {
                            values.predictions.iter().map(|prediction| {
                                html! {
                                    <div class="place-suggestion-container">
                                        <Button 
                                            name={prediction.description.clone()} 
                                            handle_click= {handle_destination_prediction_click.clone()} 
                                            class_style={
                                                if prediction.description.clone().eq(&*selected_copy_destination_state) {"place-option-button-selected".to_string()}
                                                else {"place-option-button".to_string()}
                                                }
                                        />
                                    </div>
                                }
                            }).collect()
                        }
                        None => {
                            html! {
                                <h6 class="subtitle">{"No locations yet"}</h6>
                            } 
                        }
                    }
                }
                </div>
                <h5 class="subtitle">{"Selected destination:  "}{&*selected_copy_destination_state}</h5>
                <Button name={"Calculate carbon foot print"} handle_click= {handle_calculate_click} class_style={"submit".to_string()}/>
            </div>
        </div>
    }

}