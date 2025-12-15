// src/model.rs
use serde::{Deserialize, Serialize}; 
use chrono::{DateTime, Utc};

// 1. O Enum (Estado da Célula)
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum EstadoCel {
    Agua,
    Navio,
    TiroAgua,
    TiroNavio,
}

// 2. O Tipo Tabuleiro (apenas um atalho para Vec<Vec<...>>)
pub type Tabuleiro = Vec<Vec<EstadoCel>>;

// 3. A Struct Jogo
#[derive(Serialize, Deserialize)]
pub struct Jogo {
    // Nota: Todos os campos precisam de "pub" para serem vistos no main.rs
    pub tabuleiro_jogador: Tabuleiro,
    pub tabuleiro_bot: Tabuleiro,
    pub turno_do_jogador: bool,
    pub jogo_acabou: bool,
    pub vencedor: Option<String>, 

    pub data_inicio: DateTime<Utc>,  // Guarda o carimbo de data/hora (Ex: 2025-01-01T12:00:00Z) 
    pub tempo_jogo_segundos: u64,    // Guarda quantos segundos o jogo durou
}