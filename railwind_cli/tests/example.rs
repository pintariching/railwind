// use yew::prelude::*;

// #[function_component]
// fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move |_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };

//     html! {
//         <div>
//             <div class="p-16">
//                 <button class="p-2 m-5 bg-green-600" {onclick}>{ "+1" }</button>
//                 <p class="text-xl">{ *counter }</p>
//             </div>
//         </div>
//     }
// }

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
