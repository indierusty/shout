use leptos::{error::Result, leptos_dom::logging, *};
use reqwasm::http::Headers;
use serde::Deserialize;
use web_sys::SubmitEvent;

pub fn main() {
    mount_to_body(|| {
        view! {
            <Url/>
        }
    })
}

#[derive(Deserialize, Debug)]
struct ShortUrl {
    url: String,
}

async fn create_url(url: String, email: String) -> Result<ShortUrl> {
    /* do some stuff on the server to add a new todo */
    let body = format!("{{\"url\": \"{url}\", \"email\":\"{email}\"}}");
    logging::console_log(&body);

    let res = reqwasm::http::Request::post("http://localhost:4444/api/url")
        .body(body)
        .headers({
            let header = Headers::new();
            header.append("Content-Type", "application/json");
            header
        })
        .send()
        .await?
        .json::<ShortUrl>()
        .await?;

    Ok(res)

    // if let Ok(r) = res {
    //     logging::console_log(&format!(
    //         "Successfully requested to post to make url is => {:?}",
    //         r.body().unwrap().
    //     ))
    // } else {
    //     logging::console_log("failed to post url")
    // }
}

#[component]
pub fn Url() -> impl IntoView {
    let (full_url, set_full_url) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (short_url, set_short_url) = create_signal::<Option<String>>(None);

    // if there's a single argument, just use that
    let action = create_action(move |input: &(String, String)| {
        let input = input.clone();
        async move {
            if let Ok(url) = create_url(input.0, input.1).await {
                logging::console_log(&url.url);
                set_short_url.update(|v| *v = Some(url.url));
            }
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        action.dispatch((full_url.get(), email.get()));
    };

    let pending = action.pending(); // ReadSignal<bool>

    view! {
        // use our loading state
        <p>{move || pending.get().then(|| "Creating...")}</p>
        <p>{move || short_url.get()}</p>

        <form on:submit=on_submit>
            <div>
                <input id="fullurl" type="text"
                    on:input=move |ev| { set_full_url.update(|v| *v = event_target_value(&ev)); }
                    prop:value=full_url
                />
                <label for="fullurl">Full Url</label>
            </div>
            <div>
                <input id="useremail" type="email"
                    on:input=move |ev| { set_email.update(|v| *v = event_target_value(&ev)); }
                    prop:value=email
                />
                <label for="useremail">Email</label>
            </div>
            <button type:submit>Submit</button>
        </form>
    }
}
