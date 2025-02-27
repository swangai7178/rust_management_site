/*
 *   Copyright (c) 2025 
 *   All rights reserved.
 */
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Futuristic Cookie Shop</title>
            <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
            <style>
                body {
                    background-size: cover;
                    background-attachment: fixed;
                }
                .card {
                    background: rgba(255, 255, 255, 0.8);
                    border-radius: 10px;
                    padding: 20px;
                    margin: 10px;
                    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
                }
                .card img {
                    border-radius: 10px;
                }
                .add-to-cart {
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background-color: #4CAF50;
                    color: white;
                    padding: 10px;
                    border-radius: 5px;
                    cursor: pointer;
                    transition: background-color 0.3s;
                }
                .add-to-cart:hover {
                    background-color: #45a049;
                }
                .add-to-cart svg {
                    margin-right: 5px;
                }
            </style>
        </head>
        <body class="bg-gray-100 text-gray-800">
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <div>
                        <h1 class="text-4xl font-bold text-black">Welcome to the Futuristic Cookie Shop!</h1>
                        <p class="text-lg text-black">Experience the future of cookies with our exclusive collection!</p>
                    </div>
                    <div>
                        <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2">Login</button>
                        <button class="bg-green-500 text-white px-4 py-2 rounded">Cart</button>
                    </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                    <div class="card">
                        <img src="https://sweetseidners.com/cdn/shop/products/ClassicChocChunklayers.png?v=1658975810&width=533" alt="Cookie 1" class="w-full h-48 object-cover">
                        <h2 class="text-2xl font-bold mt-2">Galactic Chocolate Chip</h2>
                        <p class="mt-1">A classic cookie with a futuristic twist, loaded with chocolate chips from another galaxy.</p>
                        <div class="add-to-cart mt-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M16 6a1 1 0 011 1v9a2 2 0 01-2 2H5a2 2 0 01-2-2V7a1 1 0 011-1h12zm-1 2H5v8h10V8zm-3-4a1 1 0 011 1v1H7V5a1 1 0 011-1h4z" />
                            </svg>
                            Add to Cart
                        </div>
                    </div>
                    <div class="card">
                        <img src="https://sweetseidners.com/cdn/shop/products/ClassicChocChunklayers.png?v=1658975810&width=533" alt="Cookie 2" class="w-full h-48 object-cover">
                        <h2 class="text-2xl font-bold mt-2">Nebula Nutty Delight</h2>
                        <p class="mt-1">A nutty delight that will take your taste buds on a journey through the stars.</p>
                        <div class="add-to-cart mt-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M16 6a1 1 0 011 1v9a2 2 0 01-2 2H5a2 2 0 01-2-2V7a1 1 0 011-1h12zm-1 2H5v8h10V8zm-3-4a1 1 0 011 1v1H7V5a1 1 0 011-1h4z" />
                            </svg>
                            Add to Cart
                        </div>
                    </div>
                    <div class="card">
                        <img src="https://sweetseidners.com/cdn/shop/products/Square-TCCMarblecomp.jpg?v=1658976640&width=260" alt="Cookie 3" class="w-full h-48 object-cover">
                        <h2 class="text-2xl font-bold mt-2">Cosmic Caramel Crunch</h2>
                        <p class="mt-1">A crunchy caramel cookie that is out of this world.</p>
                        <div class="add-to-cart mt-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M16 6a1 1 0 011 1v9a2 2 0 01-2 2H5a2 2 0 01-2-2V7a1 1 0 011-1h12zm-1 2H5v8h10V8zm-3-4a1 1 0 011 1v1H7V5a1 1 0 011-1h4z" />
                            </svg>
                            Add to Cart
                        </div>
                    </div>
                    <div class="card">
                        <img src="https://sweetseidners.com/cdn/shop/products/Square-TCCMarblecomp.jpg?v=1658976640&width=260" alt="Cookie 4" class="w-full h-48 object-cover">
                        <h2 class="text-2xl font-bold mt-2">Stellar Sugar Sprinkle</h2>
                        <p class="mt-1">A sugar cookie with sprinkles that shine like stars in the night sky.</p>
                        <div class="add-to-cart mt-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M16 6a1 1 0 011 1v9a2 2 0 01-2 2H5a2 2 0 01-2-2V7a1 1 0 011-1h12zm-1 2H5v8h10V8zm-3-4a1 1 0 011 1v1H7V5a1 1 0 011-1h4z" />
                            </svg>
                            Add to Cart
                        </div>
                    </div>
                </div>
            </div>
        </body>
        </html>
    "#;
    
    

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}