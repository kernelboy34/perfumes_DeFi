import React, { useState } from 'react';
import logo_pedra from './logo_pedra.png';
import './App.css';
import imagen1 from './img/imagenes_portada/build/perfume1_portada.png';
import imagen2 from './img/imagenes_portada/build/perfume2_portada.png';
import imagen3 from './img/imagenes_portada/build/perfume2.png';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Nav />
      </header>
      <main>
        <Body />
      </main>
    </div>
  );
}

function Nav() {
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <nav className="App-navbar">
      <img src={logo_pedra} className="App-logo" alt="logo" />
      <div className="hamburger" onClick={() => setMenuOpen(!menuOpen)}>
        ☰
      </div>
      <div className={`nav-links ${menuOpen ? 'open' : ''}`}>
        <a href="#">Start</a>
        <a href="#down">More</a>
        <a href="#">Test</a>
      </div>
    </nav>
  );
}

function Body() {
  return (
    <div className="App-body">
      <h1 className='app-letras-inicio'>make</h1>
      <h1 className='app-letras-inicio2'>your</h1>
      <h1 className='app-letras-inicio3'>own</h1>
      <h1 className='app-letras-inicio4'>perfume</h1>
      <img className="app-imagen1" src={imagen1} alt="Perfume 1" />
      <img className="app-imagen2" src={imagen2} alt="Perfume 2" />
      <a href="#down" className="scroll-down-button">⬇</a>
        <h1 id="down" className='app-letras-inicio5'>how it works?</h1>
        <p className='texto1'>we are a company that makes more easy for you all the process of make perfum with our IA and our service of blockchain everything just flow in way more easy</p>
        <img className="app-imagen3" src={imagen3} alt="Perfume 3" />
        <button className='app-button'>start</button>
    </div>
  );
}

export default App;
