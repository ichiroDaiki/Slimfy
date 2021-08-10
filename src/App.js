import React, {useEffect, useState} from 'react'
import './App.css'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'


function App() {

  useEffect(() => {
    document
      .getElementById('titlebar-minimize')
      .addEventListener('click', () => appWindow.minimize())
    document
      .getElementById('titlebar-close')
      .addEventListener('click', () => appWindow.close())
  }, [])

  const [password, SetPassword] = useState("");
  
  const generate_click = () =>{
    invoke('_generator').then((message) => SetPassword(message));
  }

  return (
    <div className="App">
      <div data-tauri-drag-region className="titlebar">
        <i className="fas fa-minus-circle" id="titlebar-minimize"></i>
        <i className="fas fa-times-circle" id="titlebar-close"></i>
      </div>

        <div className="viewpassword">
            
            <div className="title">
              <span className="title-sl">Slimfy</span>
              <span className="title-sub">a password generator</span>
            </div>

            <div className="password">
              <div className="getpassword">
                <span>
                    {
                      password !== "" ? (
                          password
                        ):(
                          ""
                        )
                    }         
                </span>
              </div>
            </div>
            
            <button className="btn-gen" onClick={()=>generate_click()}>
              Generar Contraseña
            </button>

        </div>
    </div>
  )
}

export default App
