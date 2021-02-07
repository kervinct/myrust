use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::{ConsoleService, DialogService};
use yew::prelude::*;

use crate::common::{add_auth, BACKEND_SITE};

pub struct OnePersonModel {
    link: ComponentLink<Self>,
    fetching: bool,
    ft: Option<FetchTask>,
    id: Option<u32>,
    name: String,
    can_write: bool,
    is_inserting: bool,
    go_to_persons_list_page: Option<Callback<()>>,
    username: String,
    password: String,
}

#[derive(Debug)]
pub enum OnePersonMsg {
    NameChanged(String),
    SavePressed,
    CancelPressed,
    SavedPerson,
    Failure(String),
}

#[derive(PartialEq, Clone, Properties)]
pub struct OnePersonProps {
    #[prop_or(None)]
    pub id: Option<u32>,
    #[prop_or_default]
    pub name: String,
    #[prop_or(false)]
    pub can_write: bool,
    #[prop_or(None)]
    pub go_to_persons_list_page: Option<Callback<()>>,
    #[prop_or_default]
    pub username: String,
    #[prop_or_default]
    pub password: String,
}

impl Component for OnePersonModel {
    type Message = OnePersonMsg;
    type Properties = OnePersonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetching: false,
            ft: None,
            id: props.id,
            name: props.name,
            can_write: props.can_write,
            is_inserting: props.id.is_none(),
            go_to_persons_list_page: props.go_to_persons_list_page,
            username: props.username,
            password: props.password,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.name = props.name;
        self.can_write = props.can_write;
        self.is_inserting = props.id.is_none();
        self.go_to_persons_list_page = props.go_to_persons_list_page;
        self.username = props.username;
        self.password = props.password;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use OnePersonMsg::*;
        match msg {
            NameChanged(name) => self.name = name,
            SavePressed => {
                self.fetching = true;
                let callback = self.link.callback(
                    move |response: Response<Json<Result<bool, Error>>>| {
                        let (meta, Json(_)) = response.into_parts();
                        if meta.status.is_success() {
                            SavedPerson
                        } else {
                            Failure("Cannot save the person.".to_string())
                        }
                    }
                );

                let encoded_name = url::form_urlencoded::byte_serialize(self.name.as_bytes())
                    .collect::<String>();
                let mut request = if self.is_inserting {
                    Request::post(format!("{}one_person?name={}", BACKEND_SITE, encoded_name))
                } else {
                    Request::put(format!(
                        "{}one_person?id={}&name={}",
                        BACKEND_SITE,
                        self.id.unwrap(),
                        encoded_name,
                    ))
                }
                .body(Nothing)
                .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(FetchService::fetch(request, callback).unwrap());
            }
            CancelPressed => {
                if let Some(ref go_to_page) = self.go_to_persons_list_page {
                    go_to_page.emit(());
                }
            }
            SavedPerson => {
                if let Some(ref go_to_page) = self.go_to_persons_list_page {
                    go_to_page.emit(());
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
        let change_name = self.link.callback(
            |e: InputData| OnePersonMsg::NameChanged(e.value));
        let save = self.link.callback(
            |_| OnePersonMsg::SavePressed);
        let cancel = self.link.callback(
            |_| OnePersonMsg::CancelPressed);
        html! {
            <div>
                <div>
                    <label>{ "Id: " }</label>
                    <input
                        type="number",
                        value=match self.id { 
                            Some(id) => format!("{}", id),
                            _ => "".to_string()
                        },
                        disabled=true,
                    />
                </div>
                <div>
                    <label>{ "Name: " }</label>
                    <input
                        type="text",
                        value=&self.name,
                        disabled=!self.can_write,
                        oninput=change_name,
                    />
                </div>
                <div>
                    <button
                        onclick=save,
                        disabled=!self.can_write,
                    >
                        { if self.is_inserting { "Insert" } else { "Update" } }
                    </button>
                    { " " }
                    <button
                        onclick=cancel,
                        disabled=!self.can_write,
                    >
                        { "Cancel" }
                    </button>
                </div>
            </div>
        }
    }
}
