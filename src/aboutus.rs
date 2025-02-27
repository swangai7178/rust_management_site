/*
 *   Copyright (c) 2025 
 *   All rights reserved.
 */
use actix_web::{ HttpResponse, Responder};

pub async fn about_page() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>About Us</title>
            <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
        </head>
        <body class="bg-gray-100 text-gray-800">
            <div class="container mx-auto p-4">
                <h1 class="text-4xl font-bold text-black">About Us</h1>
                <p class="text-lg text-black mt-2">Welcome to the futuristic cookie shop!</p>
                <a href="/" class="text-blue-500">Go back to Home</a>
            </div>
        </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
