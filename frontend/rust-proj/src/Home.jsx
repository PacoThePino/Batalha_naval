import { useNavigate } from 'react-router-dom';

function Home() {
    const navigate = useNavigate();

    return (
        <div style={{ 
            textAlign: 'center', marginTop: '50px',
            flexDirection: 'column', 
            alignItems: 'center',
            justifyContent: 'center', 
            height: '100vh', 
            backgroundColor: '#f0f2f5'
        }}>
            <h1 style={{ color: '#333', marginBottom: '10px',fontSize: '4rem',fontWeight: 'bold'}}>Batalha Naval ⚓</h1>
            <p style={{ color: '#666', marginBottom: '50px',fontSize: '1.5rem' }}>Bem-vindo ao melhor jogo de estratégia em Rust.</p>
            
            {/* O Botão Mágico */}
            <button 
                onClick={() => navigate('/game')}
                style={{
                    padding: '20px 50px',
                    fontSize: '1.5rem',
                    cursor: 'pointer',
                    backgroundColor: '#007bff',
                    color: 'white',
                    border: 'none',
                    borderRadius: '12px',
                    boxShadow: '0 6px 12px rgba(0,0,0,0.15)', // Sombra bonita 
                    fontWeight:'bold',
                    transition: 'transform 0.1s' // Para animar no futuro
                }}
            >
                Jogar Anonimamente
            </button>
        </div>
    );
}

export default Home;