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
        let hash = js! {
            return location.hash;
        };
        let input = parse_hash(&hash.into_string().unwrap());
        let title = format!("Mockingbob Generator: {}", input);
        js! { @(no_return)
            document.title = @{title};
        }

        Model {
            input: input,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(data) => {
                self.console
                    .log(&format!("Input changed to: {}", data.value));
                let new_title = format!("Mockingbob Generator: {}", self.input);
                js ! { @(no_return)
                    history.pushState(null, @{new_title}, "#" + @{self.input});
                    document.title = @{new_title};
                }
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

fn parse_hash(hash: &str) -> String {
    let mut split = hash.split('#').skip(1);

    let input: String = split.next().unwrap_or_default().into();
    let input_decoded =
        percent_encoding::percent_decode(input.as_bytes()).decode_utf8_lossy();

    input_decoded.into()
}
