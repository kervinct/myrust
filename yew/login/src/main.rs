use yew::prelude::*;

mod db_access;
mod login;

use login::LoginModel;
use db_access::{DbConnection, DbPrivilege, User};

enum Page {
    Login,
    PersonsList,
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
                <h2>{ "Page to be implemented" }</h2>
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
