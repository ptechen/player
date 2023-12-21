use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <div class="navigation">
      <ul>
        <li class="list active">
          <a href="#">
            <span class="icon">
              <ion-icon name="home-outline"></ion-icon>
            </span>
            <span class="text">{"Home"}</span>
          </a>
        </li>
        <li class="list">
          <a href="#">
            <span class="icon">
              <ion-icon name="person-outline"></ion-icon>
            </span>
            <span class="text">{"Profile"}</span>
          </a>
        </li>
        <li class="list">
          <a href="#">
            <span class="icon">
              <ion-icon name="chatbubble-outline"></ion-icon>
            </span>
            <span class="text">{"Message"}</span>
          </a>
        </li>
        <li class="list">
          <a href="#">
            <span class="icon">
              <ion-icon name="camera-outline"></ion-icon>
            </span>
            <span class="text">{"Photo"}</span>
          </a>
        </li>
        <li class="list">
          <a href="#">
            <span class="icon">
              <ion-icon name="settings-outline"></ion-icon>
            </span>
            <span class="text">{"Settings"}</span>
          </a>
        </li>
        <div class="indicator"></div>
      </ul>
    </div>
        )
    }
}