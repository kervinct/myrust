#![recursion_limit="512"]

use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: u64,
}

enum Msg {
    Increment,
    Reset,
    KeyDown(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Increment => { self.value += 1; true }
            Reset => { self.value = 0; true }
            KeyDown(s) => match s.as_ref() {
                "+" => {
                    self.value += 1; true
                }
                "0" => {
                    self.value = 0; true
                }
                _ => false,
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button 
                    onclick=self.link.callback(|_| Msg::Increment)>
                    {"Increment"}</button>
                <button 
                    onclick=self.link.callback(|_| Msg::Reset)>
                    {"Reset"}</button>
                <input
                    readonly=true,
                    value={self.value},
                    onkeydown=self.link.callback(|e: KeyboardEvent| Msg::KeyDown(e.key())),
                />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
