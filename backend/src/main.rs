
//imports 
use axum::{
    routing::post,
    Router, 
    Json,  
}; 

use std::net::SocketAddr;
use tower_http::cors::{Any,CorsLayer};  
use chrono::Utc; 
use model::Jogo; // Importar a struct Jogo 

//Modulos 
mod model; 
mod logic;



#[tokio::main]
async fn main() { 

    // "Any" significa: deixa entrar toda a gente (útil para dev)
    let cors = CorsLayer::new().allow_origin(Any); 
    
    // 1. Criar a nossa aplicação com uma rota simples "/"
    let app = Router::new()
        .route("/novo-jogo", post(iniciar_jogo))
        .layer(cors);

    // 2. Definir o endereço (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Batalha naval pronta em: http://{}", addr);

    // 3. Iniciar o servidor
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Esta é a função que responde quando alguém visita o site
async fn iniciar_jogo() -> Json<Jogo> {
    println!("--> A criar um novo jogo..."); 

    //Criar os tabuleiros 
    let mut tab_jogador = logic::criar_tabuleiro_vazio(); 
    let mut tab_jogador = logic::criar_tabuleiro_vazio();   

    Json(jogo)
}