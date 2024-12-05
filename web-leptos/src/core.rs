use crate::http;
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::{CrabNews, Effect, Event, ViewModel};
use std::rc::Rc;

pub type Core = Rc<shared::Core<Effect, CrabNews>>;

pub fn new() -> Core {
    Rc::new(shared::Core::new())
}

pub fn update(core: &Core, event: Event, render: WriteSignal<ViewModel>) {
    log::debug!("event: {:?}", event);

    for effect in core.process_event(event) {
        process_effect(core, effect, render);
    }
}

pub fn process_effect(core: &Core, effect: Effect, render: WriteSignal<ViewModel>) {
    log::debug!("effect: {:?}", effect);

    match effect {
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }

        Effect::Http(mut request) => {
            spawn_local({
                let core = core.clone();

                async move {
                    let response = http::request(&request.operation).await;

                    for effect in core.resolve(&mut request, response.into()) {
                        process_effect(&core, effect, render);
                    }
                }
            });
        }
    };
}
