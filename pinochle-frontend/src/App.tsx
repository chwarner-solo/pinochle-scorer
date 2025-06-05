import './App.css'

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import GamePage from "./pages/GamePage.tsx";

function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<GamePage/>}/>
                <Route path="/games/:gameId" element={<GamePage/>}/>
            </Routes>
        </BrowserRouter>
    );
}

export default App
