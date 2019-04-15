use yew::{html, prelude::*, services::ConsoleService};

pub enum Msg {
    Input(InputData),
}

pub struct Model {
    input: String,
    console: ConsoleService,
}

impl Model {
    fn space(&self) -> String {
        self.input
            .chars()
            .map(|c| {
                if rand::random() {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            input: String::new(),
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(data) => {
                self.console
                    .log(&format!("Input changed to: {}", data.value));
                self.input = data.value;
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="container",>
                <h1 id="heading",>{ "Mockingbob Generator" }</h1>
                <input type="text", id="input", oninput=|event| Msg::Input(event), />
                <h1 id="output",>{ self.space() }</h1>
            </div>
        }
    }
}
