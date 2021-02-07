use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::{ConsoleService, DialogService};
use yew::prelude::*;

use crate::common::{add_auth, Person, BACKEND_SITE};


pub struct PersonsListModel {
    link: ComponentLink<Self>,
    fetching: bool,
    ft: Option<FetchTask>,
    id_to_find: Option<u32>,
    name_portion: String,
    filtered_persons: Vec<Person>,
    selected_ids: std::collections::HashSet<u32>,
    can_write: bool,
    go_to_one_person_page: Option<Callback<Option<Person>>>,
    username: String,
    password: String,
}

#[derive(Debug)]
pub enum PersonsListMsg {
    IdChanged(String),
    FindPressed,
    PartialNameChanged(String),
    FilterPressed,
    DeletePressed,
    AddPressed,
    SelectionToggled(u32),
    EditPressed(u32),
    ReadyFilteredPersons(Result<Vec<Person>, Error>),
    ReadyDeletedPersons(Result<u32, Error>),
    ReadyPersonToEdit(Result<Person, Error>),
    Failure(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct PersonsListProps {
    #[prop_or(false)]
    pub can_write: bool,
    #[prop_or(None)]
    pub go_to_one_person_page: Option<Callback<Option<Person>>>,
    #[prop_or_default]
    pub username: String,
    #[prop_or_default]
    pub password: String,
}

impl Component for PersonsListModel {
    type Message = PersonsListMsg;
    type Properties = PersonsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Self {
            link: link,
            fetching: false,
            ft: None,
            id_to_find: None,
            name_portion: "".to_string(),
            filtered_persons: Vec::<Person>::new(),
            selected_ids: std::collections::HashSet::<u32>::new(),
            can_write: props.can_write,
            go_to_one_person_page: props.go_to_one_person_page,
            username: props.username,
            password: props.password,
        };
        model.update(PersonsListMsg::FilterPressed);
        model
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.can_write = props.can_write;
        self.go_to_one_person_page = props.go_to_one_person_page;
        self.username = props.username;
        self.password = props.password;
        self.update(PersonsListMsg::FilterPressed);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use PersonsListMsg::*;
        match msg {
            IdChanged(id_str) => self.id_to_find = id_str.parse::<u32>().ok(),
            FindPressed => match self.id_to_find {
                Some(id) => {
                    self.update(EditPressed(id));
                }
                None => {
                    DialogService::alert("No id specified.");
                }
            },
            PartialNameChanged(s) => self.name_portion = s,
            FilterPressed => {
                self.fetching = true;
                let callback = self.link.callback(
                    move |response: Response<Json<Result<Vec<Person>, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            ReadyFilteredPersons(data)
                        } else {
                            Failure("No persons found.".to_string())
                        }
                    },
                );

                let mut request = Request::get(format!(
                    "{}persons?partial_name={}",
                    BACKEND_SITE,
                    url::form_urlencoded::byte_serialize(self.name_portion.as_bytes())
                        .collect::<String>(),
                ))
                .body(Nothing)
                .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(FetchService::fetch(request, callback).unwrap());
            }
            ReadyFilteredPersons(response) => {
                self.fetching = false;
                self.filtered_persons = response.unwrap_or_else(|_| vec![]);
                ConsoleService::log(&format!(
                    "ReadyFilteredPersons: {:?}",
                    self.filtered_persons,
                ));
            }
            DeletePressed => {
                if DialogService::confirm("Do you confirm to delete the selected persons?") {
                    self.fetching = true;
                    let callback = self.link.callback(
                        move |response: Response<Json<Result<u32, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            if meta.status.is_success() {
                                ReadyDeletedPersons(data)
                            } else {
                                Failure("No persons deleted.".to_string())
                            }
                        },
                    );
                    let mut request = Request::delete(&format!(
                        "{}persons?id_list={}",
                        BACKEND_SITE,
                        self.selected_ids
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                    ))
                    .body(Nothing)
                    .unwrap();

                    add_auth(&self.username, &self.password, &mut request);
                    self.ft = Some(FetchService::fetch(request, callback).unwrap());
                }
            }
            ReadyDeletedPersons(response) => {
                self.fetching = false;
                let num_deleted = response.unwrap_or(0);
                ConsoleService::log(&format!("ReadyDeletedPersons: {}.", num_deleted));
                self.update(FilterPressed);
                DialogService::alert("Deleted.");
            }
            AddPressed => {
                if let Some(ref go_to_page) = self.go_to_one_person_page {
                    go_to_page.emit(None);
                }
            }
            SelectionToggled(id) => {
                if self.selected_ids.contains(&id) {
                    self.selected_ids.remove(&id);
                } else {
                    self.selected_ids.insert(id);
                }
            }
            EditPressed(id) => {
                self.fetching = true;
                let callback = self.link.callback(
                    move |response: Response<Json<Result<Person, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            ReadyPersonToEdit(data)
                        } else {
                            Failure("No person found with the indicated id".to_string())
                        }
                    },
                );

                let mut request = Request::get(format!("{}person/id/{}", BACKEND_SITE, id))
                    .body(Nothing)
                    .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(FetchService::fetch(request, callback).unwrap());
            }
            ReadyPersonToEdit(person) => {
                self.fetching = false;
                let person = person.unwrap_or(Person {
                    id: 0,
                    name: "".to_string(),
                });
                if let Some(ref go_to_page) = self.go_to_one_person_page {
                    go_to_page.emit(Some(person.clone()));
                }
            }
            Failure(msg) => {
                self.fetching = false;
                ConsoleService::log(&format!("Failure: {:?}", msg));
                DialogService::alert(&msg);
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let change_id = self.link.callback(|e: InputData| PersonsListMsg::IdChanged(e.value));
        let find_pressed = self.link.callback(|_| PersonsListMsg::FindPressed);
        let partial_name_change = self.link.callback(|e: InputData| PersonsListMsg::PartialNameChanged(e.value));
        let filter_pressed = self.link.callback(|_| PersonsListMsg::FilterPressed);
        let delete_pressed = self.link.callback(|_| PersonsListMsg::DeletePressed);
        let add_pressed = self.link.callback(|_| PersonsListMsg::AddPressed);
        
        let tbody = html! {
            for self.filtered_persons.iter().map(|p| {
                let id = p.id;
                let name = p.name.clone();
                html! {
                    <tr>
                        <td>
                            <input
                            type="checkbox",
                            oninput=self.link.callback(move |_| PersonsListMsg::SelectionToggled(id)),
                            checked=self.selected_ids.contains(&id),
                            /></td>
                        <td>
                        <button onclick=self.link.callback(move |_| PersonsListMsg::EditPressed(id))>{ "Edit" }</button></td>
                        <td>{ id }</td>
                        <td>{ name }</td>
                    </tr>
                }
            })
        };

        let persons_list_body = if !self.filtered_persons.is_empty() {
            html! {
                <table>
                    <thead>
                        <th></th>
                        <th></th>
                        <th>{ "Id" }</th>
                        <th>{ "Name" }</th>
                    </thead>
                    <tbody>
                    { tbody }
                    </tbody>
                </table>
            }
        } else {
            html! { <p>{ "No persons." }</p>}
        };

        html! {
            <div>
                <div>
                    <label>{ "Id: " }</label>
                    <input type="number" oninput=change_id />
                    { " " }
                    <button onclick=find_pressed>{ "Find" }</button>
                </div>
                <div>
                    <label>{ "Name portion: " }</label>
                    <input type="text" oninput=partial_name_change />
                    { " " }
                    <button onclick=filter_pressed,>{ "Filter" }</button>
                </div>
                <button onclick=delete_pressed disabled=!self.can_write>{ "Delete Selected Persons" }</button>
                { " " }
                <button onclick=add_pressed disabled=!self.can_write>{ "Add New Person" }</button>
                { persons_list_body }
            </div>
        }
    }
}
