#![recursion_limit="512"]

use yew::prelude::*;

mod db_access;
mod login;
mod one_person;
mod persons_list;

use login::LoginModel;
use db_access::{DbConnection, DbPrivilege, User, Person};
use one_person::OnePersonModel;
use persons_list::PersonsListModel;

enum Page {
    Login,
    PersonsList,
    OnePerson(Option<Person>),
}

struct MainModel {
    link: ComponentLink<Self>,
    page: Page,
    current_user: Option<String>,
    can_write: bool,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
}

enum MainMsg {
    LoggedIn(User),
    ChangeUserPressed,
    GoToOnePersonPage(Option<Person>),
    GoToPersonsListPage,
}

impl Component for MainModel {
    type Message = MainMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            page: Page::Login,
            current_user: None,
            can_write: false,
            db_connection: std::rc::Rc::new(std::cell::RefCell::new(DbConnection::new())),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use MainMsg::*;
        match msg {
            LoggedIn(user) => {
                self.page = Page::PersonsList;
                self.current_user = Some(user.username);
                self.can_write = user.privileges.contains(&DbPrivilege::CanWrite);
            }
            ChangeUserPressed => self.page = Page::Login,
            GoToOnePersonPage(person) => self.page = Page::OnePerson(person),
            GoToPersonsListPage => self.page = Page::PersonsList,
        }
        true
    }

    fn view(&self) -> Html {
        use MainMsg::*;

        let current_user = self.current_user.clone().unwrap_or("---".to_string());
        let current_page = match self.page {
            Page::Login => html! { <div/> },
            _ => html! {
                <span>
                    { " " }
                    <button
                        onclick=self.link.callback(|_| ChangeUserPressed)>
                        { "Change User" }
                    </button>
                </span>
            },
        };
        let page_body = match &self.page {
            Page::Login => html! {
                <LoginModel:
                    current_username=&self.current_user,
                    when_logged_in=self.link.callback(|user| LoggedIn(user)),
                    db_connection=Some(self.db_connection.clone()),
                />
            },
            Page::PersonsList => html! {
                // <h2>{ "Page to be implemented" }</h2>
                <PersonsListModel:
                    can_write=self.can_write,
                    go_to_one_person_page=self.link.callback(|person| GoToOnePersonPage(person)),
                    db_connection=Some(self.db_connection.clone()),
                />
            },
            Page::OnePerson(person) => html! {
                <OnePersonModel:
                    id=person.as_ref().map_or(None, |p| Some(p.id)),
                    name=person.as_ref().map_or("".to_string(), |p| p.name.clone()),
                    can_write=self.can_write,
                    go_to_persons_list_page=self.link.callback(|_| GoToPersonsListPage),
                    db_connection=Some(self.db_connection.clone()),
                />
            },
        };

        html! {
            <div>
                <style>
                { "
                    .current_user { color: #0000C0 }
                " }
                </style>
                <header>
                    <h2>{ "Person management" }</h2>
                    <p>
                        { "Current user: " }
                        <span class="current-user">
                        { current_user }
                        </span>
                        { current_page }
                    </p>
                    <hr/>
                </header>
                { page_body }
                <footer>
                    <hr/>
                    { "© Zvnlanx - Developed using Yew" }
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<MainModel>();
}
