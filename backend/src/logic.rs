use rand::{random_bool, random_range};
use crate::model::{EstadoCel, Tabuleiro}; // Importar do nosso outro ficheiro

// Função torna-se pública (pub) para o main.rs a ver
pub fn criar_tabuleiro_vazio() -> Tabuleiro {
    vec![vec![EstadoCel::Agua; 10]; 10]
}

pub fn colocar_navio_aleatorio(tabuleiro: &mut Tabuleiro, tamanho: usize) {
    
    loop { 
        
        let x = random_range(0..10);
        let y = random_range(0..10);
        let horizontal = random_bool(0.5);

        // A. Verifica Limites
        if horizontal {
            if x + tamanho > 10 { continue; }
        } else {
            if y + tamanho > 10 { continue; }
        }

        // B. Verifica Colisão
        let mut colide = false;
        for i in 0..tamanho {
            let (cx, cy) = if horizontal { (x + i, y) } else { (x, y + i) };
            // Se não for agua, é porque bateu noutro navio
            if tabuleiro[cy][cx] != EstadoCel::Agua {
                colide = true;
                break;
            }
        }
        if colide { continue; }

        // C. Grava o navio
        for i in 0..tamanho {
            let (gx, gy) = if horizontal { (x + i, y) } else { (x, y + i) };
            tabuleiro[gy][gx] = EstadoCel::Navio;
        }
        break;
    }
}