use yew::prelude::*;
use yew::services::DialogService;

use crate::db_access::{DbConnection, User};


pub struct LoginModel {
    link: ComponentLink<Self>,
    username: String,
    password: String,
    when_logged_in: Option<Callback<User>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
}

#[derive(Debug)]
pub enum LoginMsg {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoginProps {
    #[prop_or_default]
    pub current_username: Option<String>,
    #[prop_or_default]
    pub when_logged_in: Option<Callback<User>>,
    #[prop_or_default]
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Component for LoginModel {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            username: props.current_username.unwrap_or_default(),
            password: String::new(),
            when_logged_in: props.when_logged_in,
            db_connection: props.db_connection.unwrap(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.username = props.current_username.unwrap_or_default();
        self.when_logged_in = props.when_logged_in;
        self.db_connection = props.db_connection.unwrap();
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use LoginMsg::*;
        match msg {
            UsernameChanged(username) => self.username = username,
            PasswordChanged(password) => self.password = password,
            LoginPressed => {
                if let Some(user) = self
                    .db_connection
                    .borrow()
                    .get_user_by_username(&self.username)
                {
                    if user.password == self.password {
                        if let Some(ref go_to_page) = self.when_logged_in {
                            go_to_page.emit(user.clone());
                        }
                    } else {
                        DialogService::alert("Invalid password for the specified user.");
                    }
                } else {
                    DialogService::alert("User not found.");
                }
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
