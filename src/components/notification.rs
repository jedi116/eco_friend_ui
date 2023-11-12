use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub not_type: String,
    pub message: String 
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let type_of_notification = props.not_type.clone();
    let success = "success".to_string(); 
    let warning = "warning".to_string();
    let error = "error".to_string(); 
    if type_of_notification.eq(&success) {
        html! {
                <div class="success-msg">
                    <i class="fa fa-check"></i>
                </div>
        }
    } else if type_of_notification.eq(&error) {
        html! {
            <div class="error-msg">
                <i class="fa fa-times-circle"></i>
                {props.message.clone()}
            </div>
        }
    }  else if type_of_notification.eq(&warning) {
        html! {
            <div class="warning-msg">
                <i class="fa fa-warning"></i>
                {props.message.clone()}
            </div>
        }
    } else {
        html! {
        <div class="info-msg">
             <i class="fa fa-info-circle"></i>
             {props.message.clone()}
        </div>
        }
    }
    
}