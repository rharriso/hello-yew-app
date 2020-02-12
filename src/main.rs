use yew::{
    html, Callback, ClickEvent, Component, ComponentLink, Html, MouseOverEvent, Properties,
    ShouldRender,
};
use yew::services::{ ConsoleService };

struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
    onmouseover: Callback<MouseOverEvent>,
}

enum Msg {
    Click,
    MouseOver,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
            onmouseover: link.callback(|_| Msg::MouseOver),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true // Indicate that the Component should re-render
            }
            Msg::MouseOver => {
                ConsoleService::new().log("Mouse over buddy!");
                false
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };

        let image_url = if self.clicked {
            "https://i.redd.it/u0vqic0khyq31.jpg"
        } else {
            "https://i.redd.it/iw6ycbgelag41.png"
        };
        
        ConsoleService::new().debug(&format!("Sent Image url: {}", image_url));

        html! {
            <div style="display: flex; flex-direction: column;">
                <button
                    onmouseover=&self.onmouseover
                    onclick=&self.onclick>
                    {button_text }
                </button>
                <ProfilePicture image_url=String::from(image_url) />
            </div>
        }
    }
}

#[derive(Clone, Properties)]
struct ProfilePicture {
    #[props(required)]
    image_url: String,
}

impl Component for ProfilePicture {
    type Properties = ProfilePicture;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ProfilePicture {
            image_url: props.image_url,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.image_url != props.image_url;
        self.image_url = props.image_url;
        changed
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    // add code here
    fn view(&self) -> Html {
        ConsoleService::new().debug(&format!("Received Image url: {}", self.image_url));

        html! {
            <img src=&self.image_url />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
