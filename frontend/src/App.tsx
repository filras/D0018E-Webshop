import './App.css'
import json from "./assets/data.json"
import flygplan from "./assets/flygplan.png"
import TKL from "./assets/TKL.png"



interface Product{
      id: number,
      description?: string
      title: string, 
      price: string,
      discounted_price?: string,
      in_stock: number,
      average_rating?: number
}





function App() {
  return (
    <>
      <div>
        <a href="http://localhost:5173/" target="_blank">
          <img src={flygplan} className="logo" alt="flygplan" />
        </a>
        <a href="https://hianime.to/" target="_blank">
          <img src={TKL} className="logo tkl" alt="tkl logo" />
        </a>
      </div>

      <h1>
        Airplane Parts & Shieet</h1>
      <div className="card">   
      </div>

      <p className="read-the-docs">
        Click on the Airplane and Mutter logos to learn more
      </p>
        <h2>
          The greatest dev-team to ever duolingo rust!
        </h2>
      <h3 className="read-the-docs">
        Adam, Kalix, Viggo, Balto 
        </h3>
      <p> 

        {
          json.map((value: Product) =>(
            <div>
              <button> 
                <div
                    onClick={() => {
  
                      document.getElementById(value.id.toString())?.classList.toggle("hidden")      
                      
                    } }
                    key={(value.id)}>{value.title}<br></br>{value.price}

                  <div id={(value.id.toString())} className="hidden">
                  Description: {value.description}<br></br>                
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
