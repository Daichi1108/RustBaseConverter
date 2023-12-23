use yew::prelude::*;
use web_sys::HtmlInputElement;

fn main() {
    yew::start_app::<RootComponent>();
}

fn convert(inputnum:&str, inputbase:u8, outputbase:u8) -> String {
    return fromdecimal(todecimal(inputnum, inputbase), outputbase);
}

fn fromdecimal(inputnum:u128, inputbase:u8) -> String {
    let mut input = u128::from(inputnum);
    let base = u128::from(inputbase as u128);

    let mut output = String::new();

    while input > 0 {
        output.insert(0, to_char((input%base) as u8));
        input /= base;
    }

    return output;
}

fn todecimal(inputnum:&str, inputbase:u8) -> u128 {
    let input = String::from(inputnum);
    let base = u128::from(inputbase as u128);
    
    let mut output = 0;

    let mut charnum:i32 = input.len() as i32 - 1;
    
    for i in input.chars() {
        output += to_digit(i) * base.pow(charnum as u32);
        charnum -= 1;
    }

    return output;
}

fn to_char(num:u8) -> char {
    if num <= 9{
        return (num+48) as char;
    }
    return (num+55) as char;
}

fn to_digit(charinput:char) -> u128 {
    let num = u128::from(charinput as u128);

    if num <= 57 {
        return num - 48
    }
    return num - 55
}

enum Msg {
    Input(String),
    Inbase(String),
    Outbase(String),
    Enter
}
struct RootComponent {
    thing:String,
    thing1:String,
    thing2:String,
    output:String
}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { //constructor
            thing:String::new(),
            thing1:String::new(),
            thing2:String::new(),
            output:String::new()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(inputstring) => {
                self.thing = inputstring;
                true
            }
            Msg::Inbase(inputstring) => {
                self.thing1 = inputstring;
                true
            }
            Msg::Outbase(inputstring) => {
                self.thing2 = inputstring;
                true
            }
            Msg::Enter => {
                self.output = convert(&self.thing, self.thing1.parse::<u8>().unwrap_or(2), self.thing2.parse::<u8>().unwrap_or(2));
                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        html! {
            <div>
                <h1>{"Rust Base Converter"}</h1>
                <p>{"I hate ian he made me do this"}</p>
                <input class = "inputbox" type = "text" oninput = {link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value())})}/>
                <input class = "inbasebox" type = "text" oninput = {link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Inbase(input.value())})}/>
                <input class = "outbasebox" type = "text" oninput = {link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Outbase(input.value())})}/>
                <br/><br/>
                <button class = "inputbutton" onclick = {link.callback(|_|Msg::Enter)}>{"Convert"}</button>
                <div class = "outputfield">{self.output.clone()}</div>
            </div>
        }
    }
}