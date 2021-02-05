use yew::prelude::*;

use crate::db_access::{DbConnection, Person};

pub struct OnePersonModel {
    link: ComponentLink<Self>,
    id: Option<u32>,
    name: String,
    can_write: bool,
    is_inserting: bool,
    go_to_persons_list_page: Option<Callback<()>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
}

#[derive(Debug)]
pub enum OnePersonMsg {
    NameChanged(String),
    SavePressed,
    CancelPressed,
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
    #[prop_or(None)]
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Component for OnePersonModel {
    type Message = OnePersonMsg;
    type Properties = OnePersonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            id: props.id,
            name: props.name,
            can_write: props.can_write,
            is_inserting: props.id.is_none(),
            go_to_persons_list_page: props.go_to_persons_list_page,
            db_connection: props.db_connection.unwrap(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.name = props.name;
        self.can_write = props.can_write;
        self.is_inserting = props.id.is_none();
        self.go_to_persons_list_page = props.go_to_persons_list_page;
        self.db_connection = props.db_connection.unwrap();
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use OnePersonMsg::*;
        match msg {
            NameChanged(name) => self.name = name,
            SavePressed => {
                if self.is_inserting {
                    self.db_connection.borrow_mut().insert_person(Person {
                        id: 0,
                        name: self.name.clone(),
                    });
                } else {
                    self.db_connection.borrow_mut().update_person(Person {
                        id: self.id.unwrap(),
                        name: self.name.clone(),
                    });
                }
                if let Some(ref go_to_page) = self.go_to_persons_list_page {
                    go_to_page.emit(());
                }
            }
            CancelPressed => {
                if let Some(ref go_to_page) = self.go_to_persons_list_page {
                    go_to_page.emit(());
                }
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
