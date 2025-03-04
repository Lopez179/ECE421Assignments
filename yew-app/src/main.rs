use yew::prelude::*;
use meval::eval_str;

#[function_component(App)]
fn app() -> Html {
    let result: UseStateHandle<String> = use_state(|| String::new());


    let onclick = |val: &'static str| {
        let result = result.clone();
        Callback::from(move |_| {

            let key: String = val.to_string();

            if key == "ln" {
                let current_text = (*result).clone();
                let new_text = format!("ln({})", current_text);
                result.set(new_text);
            }
            else {
                let char = key.chars().next().unwrap();
                let mut new_text = (*result).clone();
                new_text.push(char);
                result.set(new_text);
            }
            
        })
    };

    let onkeydown = {
        let result = result.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key: String = e.key();
            if ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "+", "-", "*", "/"].contains(&key.as_str()) {
                let mut current_text = (*result).clone();
                current_text.push_str(&e.key());
                result.set(current_text);
            }
            else if ["Enter"].contains(&key.as_str()) {
                e.prevent_default();
                let expression_str = (*result).clone();
                let soltion = eval_str(expression_str);
                match &soltion {
                    Ok(sol) => {result.set(sol.to_string())},
                    Err(_e) => {println!("idk")},
                }
            }
            else if ["l"].contains(&key.as_str()){
                let current_text = (*result).clone();
                let new_text = format!("ln({})", current_text);
                result.set(new_text);
            }
        })
    };

    let solve = {
        let result = result.clone();
        Callback::from(move |_| {
            let expression_str = (*result).clone();
            let soltion = eval_str(expression_str);
            match &soltion {
                Ok(sol) => {result.set(sol.to_string())},
                Err(_e) => {println!("idk")},
            }
        })
    };

    let clr = {
        let result = result.clone();
        Callback::from(move |_| {
            result.set(String::new());
        })
    };

    html! {
        <html>
            <head>
                <title>{"JavaScript Calculator"}</title>
                <script src="https://cdnjs.cloudflare.com/ajax/libs/mathjs/10.6.4/math.js"
                        integrity="sha512-BbVEDjbqdN3Eow8+empLMrJlxXRj5nEitiCAK5A1pUr66+jLVejo3PmjIaucRnjlB0P9R3rBUs3g5jXc8ti+fQ=="
                        crossorigin="anonymous"
                        referrerpolicy="no-referrer"></script>
                <script src="https://cdnjs.cloudflare.com/ajax/libs/mathjs/10.6.4/math.min.js"
                        integrity="sha512-iphNRh6dPbeuPGIrQbCdbBF/qcqadKWLa35YPVfMZMHBSI6PLJh1om2xCTWhpVpmUyb4IvVS9iYnnYMkleVXLA=="
                        crossorigin="anonymous"
                        referrerpolicy="no-referrer"></script>
                //<!-- For styling -->
                <style>
                    {r#"
                        table {
                            border: 1px solid black;
                            margin-left: auto;
                            margin-right: auto;
                        }
                        input[type="button"] {
                            width: 100%;
                            padding: 20px 40px;
                            background-color: green;
                            color: white;
                            font-size: 24px;
                            font-weight: bold;
                            border: none;
                            border-radius: 5px;
                        }
                        input[type="text"] {
                            padding: 20px 30px;
                            font-size: 24px;
                            font-weight: bold;
                            border: none;
                            border-radius: 5px;
                            border: 2px solid black;
                        }
                    "#}
                </style>
                    
            </head>

            <body>
                <div style="display: flex; justify-content: center;">
                    <img src="https://media1.tenor.com/m/YxapSm-nLs8AAAAd/brah-wtf-brah.gif" alt="Logo" />
                </div>
                //<!-- Use Table to Create Calculator Structure Design -->
                <table id="calcu">
                    <tr>
                        <td colspan="3"><input type="text" id="result" value={(*result).clone()} /></td>
                        <td><input type="button" value="c" onclick={clr.clone()} /></td>
                    </tr>
                    <tr>
                        <td><input type="button" value="1" onclick={onclick("1")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="2" onclick={onclick("2")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="3" onclick={onclick("3")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="/" onclick={onclick("/")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="ln" onclick={onclick("ln")} onkeydown={onkeydown.clone()}/></td>
                    </tr>
                    <tr>
                        <td><input type="button" value="4" onclick={onclick("4")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="5" onclick={onclick("5")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="6" onclick={onclick("6")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="*" onclick={onclick("*")} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="^" onclick={onclick("^")} onkeydown={onkeydown.clone()}/></td>
                    </tr>
                    <tr>
                    <td><input type="button" value="7" onclick={onclick("7")} onkeydown={onkeydown.clone()}/></td>
                    <td><input type="button" value="8" onclick={onclick("8")} onkeydown={onkeydown.clone()}/></td>
                    <td><input type="button" value="9" onclick={onclick("9")} onkeydown={onkeydown.clone()}/></td>
                    <td><input type="button" value="-" onclick={onclick("-")} onkeydown={onkeydown.clone()}/></td>
                    <td><input type="button" value="%" onclick={onclick("%")} onkeydown={onkeydown.clone()}/></td>
                    </tr>
                    <tr>
                    <td><input type="button" value="0" onclick={onclick("0")} onkeydown={onkeydown.clone()}/></td>
                    <td><input type="button" value="." onclick={onclick(".")} onkeydown={onkeydown.clone()}/></td>
                    //<!-- solve function call function solve to evaluate value -->
                        <td><input type="button" value="=" onclick={solve} onkeydown={onkeydown.clone()}/></td>
                        <td><input type="button" value="+" onclick={onclick("+")} onkeydown={onkeydown.clone()}/></td>
                        </tr>
                </table>

            </body>
        </html>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
