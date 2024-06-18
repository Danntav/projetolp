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
                    background: url('https://wallpaperaccess.com/full/4641800.jpg') no-repeat center center fixed;
                    background-size: cover;
                    font-family: Arial, sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                }
                .container {
                    text-align: center;
                    background-color: rgba(255, 255, 255, 0.8);
                    padding: 20px;
                    border-radius: 10px;
                }
                h1 {
                    color: white;
                }
                button {
                    margin: 5px;
                    padding: 10px 20px;
                    font-size: 16px;
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
fn click(button_click: Form<ButtonClick>) -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Learn to Invest</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px;
                }
                .container {
                    text-align: center;
                }
                .content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }
                button {
                    margin: 5px;
                    padding: 10px 20px;
                    font-size: 16px;
                }
                h1 {
                    color: #333;
                }
                a {
                    text-decoration: none;
                    color: #577BFF;
                    font-size: 18px;
                }
                a:hover {
                    text-decoration: underline;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Learn to Invest</h1>
                <div class="content">
                    <a href="/education">Financial Education</a>
                    <a href="/compare">Compare CDB vs Savings</a>
                    <a href="/simulation">Investment Simulation</a>
                </div>
                <a href="/">Go back</a>
            </div>
        </body>
        </html>
    "#)
}

#[get("/education")]
fn education() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Financial Education</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px;
                }
                .container {
                    text-align: center;
                }
                .content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }
                h1 {
                    color: #333;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Financial Education</h1>
                <div class="content">
                    <p>Here you will find information on basic and advanced financial concepts.</p>
                    <ul>
                        <li><a href="/education/cdb">What is CDB?</a></li>
                        <li><a href="/education/poupanca">How does savings work?</a></li>
                        <li><a href="/education/taxas">Explanation of fees</a></li>
                    </ul>
                    <a href="/">Back to main page</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[get("/education/cdb")]
fn education_cdb() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>What is CDB?</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px;
                }
                .container {
                    text-align: center;
                }
                .content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }
                h1 {
                    color: #333;
                }
                p {
                    text-align: justify;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>What is CDB?</h1>
                <div class="content">
                    <p>The Certificate of Bank Deposit (CDB) is a security issued by banks to raise funds. By investing in a CDB, you lend money to the bank and receive interest on that amount.</p>
                    <a href="/education">Back to Financial Education</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[get("/education/poupanca")]
fn education_poupanca() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>How does savings work?</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px;
                }
                .container {
                    text-align: center;
                }
                .content {
                    display: flex;
                    flex-direction: column;
                    align-items: center.
                }
                h1 {
                    color: #333.
                }
                p {
                    text-align: justify.
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>How does savings work?</h1>
                <div class="content">
                    <p>The savings account is one of the most traditional forms of investment in Brazil. It offers a fixed return, usually based on the Selic rate or a fixed percentage.</p>
                    <a href="/education">Back to Financial Education</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[get("/education/taxas")]
fn education_taxas() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Explanation of Fees</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px.
                }
                .container {
                    text-align: center.
                }
                .content {
                    display: flex.
                    flex-direction: column.
                    align-items: center.
                }
                h1 {
                    color: #333.
                }
                p {
                    text-align: justify.
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Explanation of Fees</h1>
                <div class="content">
                    <p>There are several fees that can influence your investments, including the Selic rate, CDI, and others. Understanding these fees is crucial for making informed decisions.</p>
                    <a href="/education">Back to Financial Education</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[get("/compare")]
fn compare() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Compare CDB vs Savings</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px.
                }
                .container {
                    text-align: center.
                }
                .content {
                    display: flex.
                    flex-direction: column.
                    align-items: center.
                }
                h1 {
                    color: #333.
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Compare CDB vs Savings</h1>
                <div class="content">
                    <form action="/compare/results" method="post">
                        <label for="amount">Amount to invest:</label>
                        <input type="number" id="amount" name="amount" required>
                        <label for="duration">Duration (months):</label>
                        <input type="number" id="duration" name="duration" required>
                        <button type="submit">Compare</button>
                    </form>
                    <a href="/">Back to main page</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[derive(FromForm)]
struct CompareForm {
    amount: f64,
    duration: u32,
}

#[post("/compare/results", data = "<compare_form>")]
fn compare_results(compare_form: Form<CompareForm>) -> RawHtml<String> {
    let amount = compare_form.amount;
    let duration = compare_form.duration;
    
    // Example calculation, replace with real logic
    let cdb_return = amount * 1.1;
    let savings_return = amount * 1.05;

    RawHtml(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Comparison Results</title>
            <style>
                body {{
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px.
                }}
                .container {{
                    text-align: center.
                }}
                .content {{
                    display: flex.
                    flex-direction: column.
                    align-items: center.
                }}
                h1 {{
                    color: #333.
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Comparison Results</h1>
                <div class="content">
                    <p>Investment amount: {}</p>
                    <p>Investment duration: {} months</p>
                    <p>CDB return: {:.2}</p>
                    <p>Savings return: {:.2}</p>
                    <a href="/compare">Back to Comparison</a>
                </div>
                <a href="/">Back to main page</a>
            </div>
        </body>
        </html>
        "#,
        amount, duration, cdb_return, savings_return
    ))
}

#[get("/simulation")]
fn simulation() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Investment Simulation</title>
            <style>
                body {
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px.
                }
                .container {
                    text-align: center.
                }
                .content {
                    display: flex.
                    flex-direction: column.
                    align-items: center.
                }
                h1 {
                    color: #333.
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Investment Simulation</h1>
                <div class="content">
                    <form action="/simulate" method="post">
                        <label for="amount">Amount to invest:</label>
                        <input type="number" id="amount" name="amount" required>
                        <label for="duration">Duration (months):</label>
                        <input type="number" id="duration" name="duration" required>
                        <button type="submit">Simulate</button>
                    </form>
                    <a href="/">Back to main page</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[derive(FromForm)]
struct SimulateForm {
    amount: f64,
    duration: u32,
}

#[post("/simulate", data = "<simulate_form>")]
fn simulate(simulate_form: Form<SimulateForm>) -> RawHtml<String> {
    let amount = simulate_form.amount;
    let duration = simulate_form.duration;
    
    // Example calculation, replace with real logic
    let simulation_result = amount * 1.1;

    RawHtml(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Simulation Results</title>
            <style>
                body {{
                    background-color: #f0f0f0;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px.
                }}
                .container {{
                    text-align: center.
                }}
                .content {{
                    display: flex.
                    flex-direction: column.
                    align-items: center.
                }}
                h1 {{
                    color: #333.
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Simulation Results</h1>
                <div class="content">
                    <p>Investment amount: {}</p>
                    <p>Investment duration: {} months</p>
                    <p>Estimated return: {:.2}</p>
                    <a href="/simulation">Back to Simulation</a>
                </div>
                <a href="/">Back to main page</a>
            </div>
        </body>
        </html>
        "#,
        amount, duration, simulation_result
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, click, education, education_cdb, education_poupanca, education_taxas, compare, compare_results, simulation, simulate])
}
