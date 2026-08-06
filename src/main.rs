use leptos::prelude::*;
use leptos::mount::mount_to_body;

fn main(){
  mount_to_body(|| view! {
    <div>
      <nav class="navbar">
        <div class="nav-logo">"ROG-LOGO"</div>
        <ul class="nav-links">
          <li><a href="#home">"Home"</a></li>
          <li><a href="#about">"About"</a></li>
          <li><a href="#contact">"Contact"</a></li>
        </ul>
      </nav>

      <section class="landing">
        <h1>"Welcome ROG1! ;]"</h1>
        <p>"Simple Leptos Landing Page"</p>
        <button class="cta">"Get Started"</button>
      </section>
    </div>
  })
}
