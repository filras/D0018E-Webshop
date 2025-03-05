import './App.css'
// import json from "./assets/data.json" // mock data
import flygplan from "./assets/flygplan.png"
import TKL from "./assets/TKL.png"
import LogInn from "./assets/LogInn.png"
import bird1 from "./assets/bird1.jpg"
import buynow from "./assets/buynow.png"
import { API_URL } from "./etc/api_url";
import { useEffect, useState } from 'react'




function App() {

    const [products, setProducts] = useState<Array<Item>>([]);
    const [loading, setLoading] = useState<boolean>(true);
  
    const loadProd = async () => {
      const itemRequest = await fetch(API_URL + "/items");
      if (itemRequest.ok) {
        const items: Array<Item> = await itemRequest.json();
        setProducts(items);
      }
      setLoading(false);
    }
    // Load products once at start of session
    useEffect(() => {loadProd();}, [])
  
  return (
    <div className="homepage">
    <div> 
    
      <div>
      <h1>
      Airplane Parts & Shieet
      </h1>

        <a href="/">
          <img src={flygplan} className='logo' alt='flygplan'/>
        </a>

      </div>
      <div>
        <a href="/login">
          <img src={LogInn} className="logo" alt="loginn" />
        </a>
        <a href="/register">
          <img src={TKL} className="logo" alt="tkl logo" />
        </a>
        <div>
          <p className="read-the-docs">
            Click on the Log Inn to log in and mutter to create an account
          </p>
          </div>
        </div>
      </div> 
    
      <div className='item3'>
        <h2>
          The greatest dev-team to ever duolingo rust!
        </h2>
      <h3 className="read-the-docs">
        Adam, Kalix, Viggo, Balto 
        </h3>
    </div>

      
    <div className='card-container'> 
      
       
      {
          !loading && products.map((value: Item) =>(
            
            <div className='card'>
              
              <img src={bird1} className="bird-logo" alt="bird-logo"/>
              
              <button className='card-button'> 
                <div
                    onClick={() => {

                      document.getElementById(value.id.toString())?.classList.toggle("hidden")      
                      
                    } } 
                    key={(value.id)}>{value.title}
                    <div className='price'>
                    {value.price+"  ₺"}
                    </div>
                  <div id={(value.id.toString())} className="hidden">
                  Description: {value.description}<br></br>                
                  Discount: {value.discounted_price}<br></br>
                  Rating: {value.average_rating}<br></br>
                  Stock: {value.in_stock}

                  </div>
                </div>
              </button>
              <img src={buynow} className='buynow-pic' alt="buynow"
              onClick={()=> {
                

                 // Add item to shopping cart 
                
              }}></img>
                            
              <br></br>
            </div>  
          ))}
      
        </div>
                  
    </div>
  )
}

export default App
