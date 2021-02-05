use yew::prelude::*;

use yew::services::DialogService;

use crate::db_access::{DbConnection, Person};


pub struct PersonsListModel {
    link: ComponentLink<Self>,
    id_to_find: Option<u32>,
    name_portion: String,
    filtered_persons: Vec<Person>,
    selected_ids: std::collections::HashSet<u32>,
    can_write: bool,
    go_to_one_person_page: Option<Callaback<Option<Person>>>,
    db_connection: std::rc::Rc<std::cell::RefCel<DbConnection>>,
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
}

#[derive(Clone, PartialEq, Properties)]
pub struct PersonsListProps {
    #[prop_or(false)]
    pub can_write: bool,
    #[prop_or(None)]
    pub go_to_one_person_page: Option<Callback<Option<Person>>>,
    #[prop_or(None)]
    pub db_connection: Option<std::rc::Rc<std::cell::Cell<DbConnection>>>,
}

impl Component for PersonsListModel {
    type Message = PersonsListMsg;
    type Properties = PersonsListProps;

    pub fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Self {
            link: link,
            id_to_find: None,
            name_portion: "".to_string(),
            filtered_persons: Vec::<Person>::new(),
            selected_ids: std::collections::HashSet::<u32>::new(),
            can_write: props.can_write,
            go_to_one_person_page: props.go_to_one_person_page,
            db_connection: props.db_connection.unwrap(),
        };
        model.filtered_persons = model.db_connection
            .borrow()
            .get_persons_by_partial_name("");
        model
    }

    pub fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.can_write = props.can_write;
        self.go_to_one_person_page = props.go_to_one_person_page;
        self.db_connection = props.db_connection.unwrap(),
        self.filtered_persons = self.db_connection
            .borrow()
            .get_persons_by_partial_name("");
        true
    }

    pub fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                self.filtered_persons = self.db_connection
                    .borrow()
                    .get_persons_by_partial_name(&self.name_portion);
            }
            DeletePressed => {
                if DialogService::confirm("Do you confirm to delete the selected persons?") {
                    {
                        let mut db = self.db_connection.borrow_mut();
                        for id in &self.selected_ids {
                            db.delete_by_id(*id);
                        }
                    }
                    self.update(FilterPressed);
                    DialogService.alert("Deleted.");
                }
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
                match self.db_connection.borrow().get_person_by_id(id) {
                    Some(person) => {
                        if let Some(ref go_to_page) = self.go_to_one_person_page {
                            go_to_page.emit(Some(person.clone()));
                        }
                    }
                    None => DialogService::alert("No person found with the indicated id."),
                }
            }
        }
        true
    }

    pub fn view(&self) -> Html {
        html! {
	    <div>
                <div>
                    <label>{ "Id: " }</label>
                    <input
                        type="number",
                        oninput=|e| PersonsListMsg::IdChanged(e.value),/>
                    { " " }
                    <button
                        onclick=|_| PersonsListMsg::FindPressed,>
                        { "Find" }
                    </button>
                </div>
                <div>
                    <label>{ "Name portion: " }</label>
                    <input
                        type="text",
                        oninput=|e| PersonsListMsg::PartialNameChanged(e.value),
                    />
                    { " " }
                    <button
                        onclick=|_| PersonsListMsg::FilterPressed,
                    >
                        { "Filter" }
                    </button>
                </div>
                <button
                    onclick=|_| PersonsListMsg::DeletePressed,
                    disabled=!self.can_write,
                >
                    { "Delete Selected Persons" }
                    </button>
                { " " }
                <button
                    onclick=|_| PersonsListMsg::AddPressed,
                    disabled=!self.can_write,
                >
                    { "Add New Person" }
                    </button>

                {
                    if !self.filtered_persons.is_empty() {
                        html! {
                            <table>
                                <thead>
                                    <th></th>
                                    <th></th>
                                    <th>{ "Id" }</th>
                                    <th>{ "Name" }</th>
                                </thead>
                                <tbody>
                                    {
                                        for self.filtered_persons.iter().map(|p| {
                                            let id = p.id;
                                            let name = p.name.clone();
                                            html! {
                                                <tr>
                                                    <td><input
                                                        type="checkbox",
                                                        oninput=|_| PersonsListMsg::SelectionToggled(id),
                                                        checked=self.selected_ids.contains(&id),
                                                        /></td>
                                                    <td><button
                                                        onclick=|_| PersonsListMsg::EditPressed(id),>{ "Edit" }</button></td>
                                                    <td>{ id }</td>
                                                    <td>{ name }</td>
                                                </tr>
                                            }
                                        })
                                    }
                                </tbody>
                            </table>
                        }
                    }
                    else {

                            <p>{ "No persons." }</p>
                        }
                    }
                }
            </div>
        }
    }
}
