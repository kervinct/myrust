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
    go_to_one_person_page: Option<Callback<Option<Person>>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
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
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Component for PersonsListModel {
    type Message = PersonsListMsg;
    type Properties = PersonsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.can_write = props.can_write;
        self.go_to_one_person_page = props.go_to_one_person_page;
        self.db_connection = props.db_connection.unwrap();
        self.filtered_persons = self.db_connection
            .borrow()
            .get_persons_by_partial_name("");
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
                    DialogService::alert("Deleted.");
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
