import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Home from './Home';
import Game from './Game';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Quando o URL for "/", mostra o componente Home */}
        <Route path="/" element={<Home />} />
        
        {/* Quando o URL for "/game", mostra o componente Game */}
        <Route path="/game" element={<Game />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;