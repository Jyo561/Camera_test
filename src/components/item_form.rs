use yew::prelude::*; 
use crate::models::Item; 

#[derive(Properties, PartialEq)] 
pub struct ItemFormProps { 
    pub item: Option<Item>, 
    pub on_save: Callback<Item>, 
    pub on_cancel: Callback<()>, 
} 

#[function_component(ItemForm)] 
pub fn item_form(props: &ItemFormProps) -> Html { 
    let existing = props.item.clone(); 
    let name = use_state(|| { existing .as_ref() .map(|x| x.name.clone()) .unwrap_or_default() }); 
    let location = use_state(|| { existing .as_ref() .map(|x| x.location.clone()) .unwrap_or_default() }); 
    let category = use_state(|| { existing .as_ref() .map(|x| x.category.clone()) .unwrap_or_default() }); 
    let notes = use_state(|| { existing .as_ref() .map(|x| x.notes.clone()) .unwrap_or_default() });
    let photo = use_state(|| { existing .as_ref() .map(|x| x.photo_path.clone()). unwrap_or_default() });
    let existing2 = existing.clone();
    let value = photo.clone();
    let on_submit = { 
        let name = name.clone(); 
        let location = location.clone(); 
        let category = category.clone(); 
        let notes = notes.clone(); 
        let callback = props.on_save.clone(); 
        let id = existing.as_ref().map(|x| x.id).unwrap_or(0); 
        Callback::from(move |e: SubmitEvent| { 
            e.prevent_default(); 
            let item = Item { 
                id, 
                name: (*name).clone(), 
                location: (*location).clone(), 
                category: (*category).clone(), 
                notes: (*notes).clone(),
                photo_path: (*value).clone(),
            }; 
            callback.emit(item); 
        }) 
    }; 

    let on_take_photo = {
        let photo = photo.clone();

        Callback::from(move |_| {
            let photo = photo.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match crate::camera::take_photo().await {
                    Ok(image) => {
                        photo.set(Some(image));
                    }

                    Err(error) => {
                        web_sys::console::error_1(&error);
                    }
                }
            });
        })
    };

    html! { 
        <div class="modal"> 
            <div class="modal-content"> 
                <h2> { if existing.is_some() { "Edit Item" } else { "Add Item" } } </h2> 
                <form onsubmit={on_submit}> 
                    <label> 
                        {"Name"} 
                        <input value={(*name).clone()} oninput={{ let name = name.clone(); Callback::from(move |e: InputEvent| { let input: web_sys::HtmlInputElement = e.target_unchecked_into(); name.set(input.value()); }) }} placeholder="Sony Headphones" /> 
                    </label> 
                    
                    <label> {"Where is it?"} 
                        <input value={(*location).clone()} oninput={{ let location = location.clone(); Callback::from(move |e: InputEvent| { let input: web_sys::HtmlInputElement = e.target_unchecked_into(); location.set(input.value()); }) }} placeholder="Desk drawer" /> 
                    </label> 

                    <label> {"Category"} 
                        <input value={(*category).clone()} oninput={{ let category = category.clone(); Callback::from(move |e: InputEvent| { let input: web_sys::HtmlInputElement = e.target_unchecked_into(); category.set(input.value()); }) }} placeholder="Electronics" /> 
                    </label> 

                    <label> {"Notes"} 
                        <textarea value={(*notes).clone()} oninput={{ let notes = notes.clone(); Callback::from(move |e: InputEvent| { let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into(); notes.set(input.value()); }) }} /> 
                    </label> 
                    
                    <div class="photo-section">

                        <label>
                            {"Photo"}
                        </label>

                       {
                            if let Some(photo_path) = &*photo {

                                html! {
                                    <div class="photo-preview">

                                        <img src={photo_path.clone()} alt="Item" />

                                        <button type="button"
                                            onclick={
                                                {   
                                                    let photo = photo.clone();

                                                    Callback::from(move |_| {
                                                        photo.set(None);
                                                    })
                                                }
                                            }
                                        >
                                            {"Remove Photo"}
                                        </button>

                                    </div>
                                }

                            } else {

                                html! {
                                    <div class="photo-placeholder">

                                        <div class="photo-icon">
                                            {"📷"}
                                        </div>

                                        <p>
                                            {"No photo added"}
                                        </p>

                                        <button type="button" onclick={on_take_photo}>
                                            {"Take Photo"}
                                        </button>

                                    </div>
                                }
                            }
                        }

                    </div>

                    <div class="form-actions"> 
                        <button type="button" onclick={{ let callback = props.on_cancel.clone(); Callback::from(move |_| { callback.emit(()); }) }} > {"Cancel"} </button> 
                        <button type="submit"> {"Save"} </button> 
                    </div> 
                </form> 
            </div> 
        </div> 
    } 
}
