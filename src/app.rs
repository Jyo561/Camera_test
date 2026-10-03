use yew::prelude::*;

use crate::components::item_card::ItemCard;
use crate::components::item_form::ItemForm;
use crate::models::Item;

use crate::storage;

#[function_component(App)]
pub fn app() -> Html {

    let items = use_state(Vec::<Item>::new);
    let search = use_state(String::new);

    let show_form = use_state(|| false);

    let editing_item = use_state(|| None::<Item>);
    
    {
        let items = items.clone();

        use_effect_with((), move |_| {
            match storage::load_items() {
                Ok(data) => items.set(data),

                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }

            || ()
        });
    }

    let on_search = {

        let search = search.clone();

        Callback::from(move |e: InputEvent| {

            let input: web_sys::HtmlInputElement =
                e.target_unchecked_into();

            search.set(input.value());
        })
    };

    let on_delete = {

        let items = items.clone();

        Callback::from(move |id: u32| {
            match storage::delete_item(id) {
                Ok(updated) => {
                    items.set(updated);
                }

                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }
        })
    };

    let on_edit = {

        let editing_item = editing_item.clone();
        let show_form = show_form.clone();

        Callback::from(move |item: Item| {

            editing_item.set(Some(item));
            show_form.set(true);
        })
    };

    let on_add = {

        let editing_item = editing_item.clone();
        let show_form = show_form.clone();

        Callback::from(move |_| {

            editing_item.set(None);
            show_form.set(true);
        })
    };

    let on_save = {

        let items = items.clone();

        //let editing_item = editing_item.clone();
        let show_form = show_form.clone();

        Callback::from(move |mut item: Item| {

            if item.id == 0 {
                match storage::next_id() {
                    Ok(id) => item.id = id,

                    Err(error) => {
                        web_sys::console::error_1(&error);
                        return;
                    }
                }
            }


            let result = if items.iter().any(|x| x.id == item.id) {
                storage::update_item(item)
            } else {
                storage::add_item(item)
            };

            match result {
                Ok(updated) => {
                    items.set(updated);
                    show_form.set(false);
                }

                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }
        })
    };

    let filtered_items = items
        .iter()
        .filter(|item| {

            if search.is_empty() {
                return true;
            }

            let query = search.to_lowercase();

            item.name.to_lowercase().contains(&query)
                || item.location.to_lowercase().contains(&query)
                || item.category.to_lowercase().contains(&query)
        })
        .cloned()
        .collect::<Vec<_>>();

    html! {

        <div class="app">

            <header>

                <div>
                    <h1>
                        {"Find My Stuff"}
                    </h1>

                    <p>
                        {"Remember where you keep everything."}
                    </p>
                </div>

                <button
                    class="add-button"
                    onclick={on_add}
                >
                    {"+ Add"}
                </button>

            </header>

            <div class="search">

                <span>
                    {"🔍"}
                </span>

                <input
                    type="text"
                    placeholder="Search your stuff..."
                    value={(*search).clone()}
                    oninput={on_search}
                />

            </div>

            <main>

                {
                    if filtered_items.is_empty() {

                        html! {
                            <div class="empty">
                                <h2>
                                    {"Nothing found"}
                                </h2>

                                <p>
                                    {"Try another search or add something."}
                                </p>
                            </div>
                        }

                    } else {

                        html! {
                            <div class="items">

                                {
                                    filtered_items
                                        .into_iter()
                                        .map(|item| {

                                            html! {
                                                <ItemCard
                                                    key={item.id}
                                                    item={item.clone()}
                                                    on_delete={on_delete.clone()}
                                                    on_edit={on_edit.clone()}
                                                />
                                            }

                                        })
                                        .collect::<Html>()
                                }

                            </div>
                        }
                    }
                }

            </main>

            if *show_form {

                <ItemForm
                    item={(*editing_item).clone()}
                    on_save={on_save}
                    on_cancel={{
                        let show_form = show_form.clone();

                        Callback::from(move |_| {
                            show_form.set(false);
                        })
                    }}
                />

            }

        </div>
    }
}
