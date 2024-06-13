#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::serde::{Deserialize, Serialize};

#[derive(FromForm, Deserialize, Serialize)]
struct ButtonClick {
    button: String,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>App Investidor</title>
            <style>
                body {
                    background-image: url('https://wallpaperaccess.com/full/4641800.jpg');
                    background-size: cover;
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    color: white;
                }
                .container {
                    text-align: center;
                    color: #000000;
                }
                button {
                    margin: 5px;
                    padding: 10px 20px;
                    font-size: 16px;
                }
                h1 {
                    color: white; /* Text color for the h1 element */
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Aprenda a investir!</h1>
                <form action="/click" method="post">
                    <button name="button" value="start_button">Start</button>
                </form>
            </div>
        </body>
        </html>
    "#)
}

#[post("/click", data = "<button_click>")]
fn click(button_click: Form<ButtonClick>) -> RawHtml<String> {
    RawHtml(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>App Investidor</title>
            <style>
                body {{
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                }}
                .container {{
                    text-align: center;
                }}
                a {{
                    display: inline-block;
                    margin-top: 20px;
                    text-decoration: none;
                    color: #577BFF;
                    font-size: 18px;
                }}
                a:hover {{
                    text-decoration: underline;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>You clicked: {}</h1>
                <a href='/'>Go back</a>
            </div>
        </body>
        </html>
        "#,
        button_click.button
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, click])
}
