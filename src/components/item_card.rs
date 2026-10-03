use yew::prelude::*; 
use crate::models::Item; 

#[derive(Properties, PartialEq)] 
pub struct ItemCardProps { 
    pub item: Item, 
    pub on_delete: Callback<u32>, 
    pub on_edit: Callback<Item>, 
} 

#[function_component(ItemCard)] 
pub fn item_card(props: &ItemCardProps) -> Html { 
    let item = &props.item; 
    let delete_id = item.id; 
    let on_delete = { 
        let callback = props.on_delete.clone(); 
        Callback::from(move |_| { 
            callback.emit(delete_id); 
        }) 
    }; 
    let edit_item = item.clone(); 
    let on_edit = { 
        let callback = props.on_edit.clone(); 
        Callback::from(move |_| { 
            callback.emit(edit_item.clone()); 
        }) 
    }; 

    html! { 
        <div class="item-card"> 
            <div class="item-icon"> {"📦"} </div> 
            <div class="item-info"> 
                <h3> { &item.name } </h3> 
                <div class="item-location"> {"📍 "} { &item.location } </div> 
                <div class="item-category"> {"🏷️ "} { &item.category } </div> 
                if !item.notes.is_empty() { 
                    <p class="item-notes"> { &item.notes } </p> 
                } 
            </div> 
            <div class="item-actions"> 
                <button onclick={on_edit}> {"✏️"} </button> 
                <button onclick={on_delete}> {"🗑️"} </button> 
            </div> 
        </div> 
    } 
}
