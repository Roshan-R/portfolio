use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="box-container">
            <div class="box">
                <span class="hi">{"Hi"}</span> <br/>
                <span class="me">{"I'm"} </span>
                <span class="Roshan">{" Roshan"}</span>
            </div>
            <div class="socials-container">
                <a class="ref" href="https://rosh.netlify.app" target="_blank">{"Blog"}</a>
                <a class="ref" href="https://www.linkedin.com/in/roshan-r-chandar-810787206" target="_blank"><span class="ref">{"LinkedIn"}</span></a>
                <a class="ref" href="https://github.com/Roshan-R" target="_blank"><span class="ref">{"Github"}</span></a>
                <a href="mailto:roshan@cet.ac.in">{"Email"}</a>
                <a href="Resume.pdf">{"Resume"}</a>
            </div>

            <footer class="footer">  
                {"Made with ♥  using"}<a class="yew" href="https://yew.rs/">{"yew.rs"}</a>
            </footer>  

        </div>

    }
}

fn main() {
    yew::start_app::<App>();
}
