import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import json from "./assets/data.json"

interface Props{text: string, color: string}

interface Product{
      id: number,
      description?: string
      title: string, 
      price: string,
      discounted_price?: string,
      in_stock: number,
      average_rating?: number
}

function Test({text, color}: Props){
  return (
    <div style={{backgroundColor: color}}>
      {text}
    </div>
  )
}

const arr = ["Adam", "Viggo", "Kalix", "Balto", "", "", ""];

function showInfo(is_visable: Boolean, value: Product) {
  if(is_visable = true)
  value.description
}




function App() {
  const [count, setCount] = useState(0)
  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
        <Test text={"The Greatest Dev-team to ever duolingo Rust"} color={"Black"}/>
      <p>
        {arr.map(value=><Test text={value} color={value==="Bengt"? "Green": "Red"}/>)}
        </p>
      <p> 

        {
          json.map((value: Product) =>(
            <div>
              <button> 
                <div  
                    onClick={() => {
  
                      document.getElementById(value.id.toString())?.classList.toggle("hidden")      
                      
                    } }
                    key={(value.id)}>{value.title}

                  <div id={(value.id.toString())} className="hidden">
                  Description: {value.description}<br></br>
                  Price: {value.price}<br></br>
                  Discount: {value.discounted_price}<br></br>
                  Rating: {value.average_rating}<br></br>
                  Stock: {value.in_stock}

                  </div>
                </div>
              </button>
            </div>  
          ))}
        </p>    
    </>
  )
}

export default App
