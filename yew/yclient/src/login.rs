use anyhow::Error;
use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::{DialogService, ConsoleService};
use yew::prelude::*;

use crate::common::{add_auth, User, BACKEND_SITE};

pub struct LoginModel {
    link: ComponentLink<Self>,
    fetching: bool,
    ft: Option<FetchTask>,
    username: String,
    password: String,
    when_logged_in: Option<Callback<User>>,
}

#[derive(Debug)]
pub enum LoginMsg {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
    ReadyLogin(User),
    Failure(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoginProps {
    #[prop_or_default]
    pub username: String,
    #[prop_or_default]
    pub password: String,
    #[prop_or_default]
    pub when_logged_in: Option<Callback<User>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthenticationResult {
    user: User,
}

impl Component for LoginModel {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetching: false,
            ft: None,
            username: props.username,
            password: props.password,
            when_logged_in: props.when_logged_in,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.username = props.username;
        self.password = props.password;
        self.when_logged_in = props.when_logged_in;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use LoginMsg::*;
        match msg {
            UsernameChanged(username) => self.username = username,
            PasswordChanged(password) => self.password = password,
            LoginPressed => {
                if self.username.is_empty() {
                    DialogService::alert("User not specified.");
                    return false;
                }
                self.fetching = true;
                let callback = self.link.callback(
                    move |response: Response<Json<Result<AuthenticationResult, Error>>>| {
                        let (_, Json(data)) = response.into_parts();
                        if data.is_ok() {
                            ReadyLogin(data.unwrap().user)
                        } else {
                            Failure("".to_string())
                        }
                    },
                );
                let mut request = Request::get(format!("{}authenticate", BACKEND_SITE))
                    .body(Nothing)
                    .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(FetchService::fetch(request, callback)
                    .expect("failed to start request"));
            }
            ReadyLogin(user) => {
                self.fetching = false;
                ConsoleService::log(&format!("User: {:?}", user));
                if let Some(ref go_to_page) = self.when_logged_in {
                    go_to_page.emit(user.clone());
                }
            }
            Failure(msg) => {
                self.fetching = false;
                ConsoleService::log(&format!("Failure: {:?}.", msg));
                DialogService::alert(&msg);
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let change_username = |e: InputData| LoginMsg::UsernameChanged(e.value);
        let change_password = |e: InputData| LoginMsg::PasswordChanged(e.value);
        let login_pressed   = |_| LoginMsg::LoginPressed;
        html! {
            <div>
                <div>
                    <label>{ "User name:" }</label>
                    <input
                        type="text",
                        value=&self.username,
                        oninput=self.link.callback(change_username),
                    />
                </div>
                <div>
                    <label>{ "Password:" }</label>
                    <input
                        type="password",
                        oninput=self.link.callback(change_password),
                    />
                </div>
                <button onclick=self.link.callback(login_pressed)>
                    { "Login in" }
                </button>
            </div>
        }
    }
}
