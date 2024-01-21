use yew::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct ReportPageProps {
    pub origin: String,
    pub destination: String
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CalculatedTranportCarbonFootPrint {
    name: String,
    t_type: String,
    fuel: String,
    unit: String,
    region: String,
    footprint: f64
}


#[derive(Debug, Deserialize, Serialize, Clone)]
struct ReportResponse {
    report: Vec<CalculatedTranportCarbonFootPrint>
}


#[derive(Debug, Deserialize, Serialize)]
struct ReportRequest {
    origin_address: String,
    destination_address: String
}

#[function_component(Report)]
pub fn report(props: &ReportPageProps) -> Html {

    let report_state: UseStateHandle<Option<Vec<CalculatedTranportCarbonFootPrint>>>  = use_state(|| Some(vec![]));
    let report_state_clone = report_state.clone();

    let report_state_view_data = report_state.clone();

    let borrowed_props = props.clone();
    let request_json = serde_json::to_value(ReportRequest {
        origin_address: borrowed_props.origin.clone(),
        destination_address: borrowed_props.destination.clone()
       }).unwrap().to_string();
       log!(JsValue::from(&request_json));

    use_effect_with([props.destination.clone(), props.origin.clone()], move |_| {
        spawn_local(async move {
            let response: Vec<CalculatedTranportCarbonFootPrint> = match reqwasm::http::Request::post("http://localhost:8080/getCarbonFootPrint")
            .header("Content-Type", "application/json")
            .body( JsValue::from(request_json))
            .send()
            .await
            .unwrap()
            .json()
            .await {
                Ok(data) => data,
                Err(error) => {
                    let error_message = format!("{:?}", error);
                log!(JsValue::from_str(&error_message));
                    vec![]
                }
            };
            let mut sorted_repsponse = response.clone();
            sorted_repsponse.sort_by(|a, b| a.footprint.partial_cmp(&b.footprint).unwrap());
            &report_state_clone.set(Some(sorted_repsponse));
           
    
        });
    });   
    
    html! {
        <div class="report-page">
            <h1>{"Report"}</h1>
            <table class="styled-table">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Vechicle Type"}</th>
                        <th>{"Fuel"}</th>
                        <th>{"carbon footprint"}</th>
                        <th>{"unit"}</th>
                    </tr>
                </thead>
                    <tbody>
                        {
                            match &*report_state_view_data {
                                Some(values) => {
                                    values.iter().map(|report_data| {
                                        html! {
                                            <tr>
                                                <td>{report_data.name.clone()}</td>
                                                <td>{report_data.t_type.clone()}</td>
                                                <td>{report_data.fuel.clone()}</td>
                                                <td>{report_data.footprint.clone()}</td>
                                                <td>{report_data.unit.clone()}</td>
                                            </tr>
                                        }
                                    }).collect()
                                }
                                None => {
                                    html! {
                                        <h6 class="subtitle">{"No report data"}</h6>
                                    } 
                                }
                            }
                        }
                    </tbody>
            </table>
        </div>
    }
}