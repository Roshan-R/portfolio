use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
         <footer class="footer">
             {"made with ♥  using"}<a class="yew" href="https://yew.rs/">{"yew.rs"}</a>
         </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="box-container">
            <div class="box">
                <div class="contents">
                    <span class="hi">{"Hi"}</span> <br/>
                    <span class="me">{"I'm"} </span>
                    <span class="Roshan">{" Roshan"}</span>
                </div>
                // <a class="ref overlay" href="https://github.com/Roshan-R/portfolio" target="_blank">{"Github"}</a>
            </div>
            <div class="socials-container">
                <a class="ref" href="https://rosh.netlify.app" target="_blank">{"Blog"}</a>
                <a class="ref" href="https://www.linkedin.com/in/roshan-r-chandar-810787206" target="_blank"><span class="ref">{"LinkedIn"}</span></a>
                <a class="ref" href="https://github.com/Roshan-R" target="_blank"><span class="ref">{"Github"}</span></a>
                <a class="ref" href="https://gitlab.gnome.org/Roshan-R" target="_blank"><span class="ref">{"GitLab"}</span></a>
                <a href="mailto:roshan@cet.ac.in">{"Email"}</a>
                <a href="Resume.pdf">{"Resume"}</a>
            </div>

            <Footer>
            </Footer>

        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
