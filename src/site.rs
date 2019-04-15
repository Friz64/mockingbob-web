use yew::{html, prelude::*, services::ConsoleService};

pub enum Msg {
    Input(InputData),
}

pub struct Model {
    input: String,
    console: ConsoleService,
}

impl Model {
    fn mock(&self) -> String {
        self.input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
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
              <div id="wrapper",>
                <input type="text", placeholder="Input", id="input", class="line", oninput=|event| Msg::Input(event), />
                <div id="output", class="line",>{ self.mock() }</div>
              </div>
            </div>
        }
    }
}
