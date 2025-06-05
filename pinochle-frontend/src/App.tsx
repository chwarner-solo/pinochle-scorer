import './App.css'

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import GamePage from "./pages/GamePage.tsx";
import HandPage from "./pages/HandPage.tsx";

function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<GamePage/>}/>
                <Route path="/games/:gameId" element={<HandPage/>}/>
            </Routes>
        </BrowserRouter>
    );
}

export default App
