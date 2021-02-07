#![recursion_limit="512"]

use yew::prelude::*;

mod common;
mod login;
mod one_person;
mod persons_list;

use login::LoginModel;
use common::{DbPrivilege, User, Person};
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
    can_write: bool,
    username: String,
    password: String,
}

#[derive(Debug)]
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
            can_write: false,
            username: "".to_string(),
            password: "".to_string(),
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
                self.username = user.username;
                self.password = user.password;
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

        let current_user = if self.username.is_empty() {
            "---"
        } else {
            &self.username
        };
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
                    username=self.username.clone(),
                    password=self.password.clone(),
                    when_logged_in=self.link.callback(|user| LoggedIn(user)),
                />
            },
            Page::PersonsList => html! {
                // <h2>{ "Page to be implemented" }</h2>
                <PersonsListModel:
                    can_write=self.can_write,
                    go_to_one_person_page=self.link.callback(|person| GoToOnePersonPage(person)),
                    username=self.username.clone(),
                    password=self.password.clone(),
                />
            },
            Page::OnePerson(person) => html! {
                <OnePersonModel:
                    id=person.as_ref().map_or(None, |p| Some(p.id)),
                    name=person.as_ref().map_or("".to_string(), |p| p.name.clone()),
                    can_write=self.can_write,
                    go_to_persons_list_page=self.link.callback(|_| GoToPersonsListPage),
                    username=self.username.clone(),
                    password=self.password.clone(),
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
