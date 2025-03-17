import './App.css';
import bird1 from "./assets/bird1.jpg";
import { API_URL } from "./etc/api_url";
import { useEffect, useState } from 'react';
import { Link } from 'react-router';
import { ToastContainer, toast } from "react-toastify";
import { CURRENCY } from './etc/const';
import moreInfo from "./assets/moreInfo.png";
import { AuthUser } from './auth/ProtectedRoute';

interface Props {
  user: AuthUser | null;
  loadingUser: boolean;
}; 
function App({user, loadingUser}: Props) {

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
  useEffect(() => { loadProd(); }, [])

  async function search(e: any){
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);
    
    //Get the string out from the object
    const formres = Object.fromEntries(formData.entries());
    const stringValues = Object.entries(formres)
    let stringExtraction = stringValues[0][1];
    
 
    // Rerenders the items to match the search
    const finalfetch = await fetch(API_URL + "/items?search="+stringExtraction);
    if (finalfetch.ok){
      const items: Array<Item> = await finalfetch.json();
      setProducts(items); 
    }
  }

  async function sortName(){
   const finalNameSort =  await fetch(API_URL + "/items?sort_by=Name");
    if (finalNameSort.ok){
      const items: Array<Item> = await finalNameSort.json();
      setProducts(items); 
    }
  }

  async function sortPrice(){
    const finalPriceSort =  await fetch(API_URL + "/items?sort_by=Price");
     if (finalPriceSort.ok){
       const items: Array<Item> = await finalPriceSort.json();
       setProducts(items); 
     }
   }

  return (
    <div className="homepage">
      <div>
        <div>
          <h1>
            Airplane Parts & Shieet
          </h1>
            { !loadingUser && !user && ( <>
              
                <p className="read-the-docs">
                  Click on the Log Inn to log in and mutter to create an account
                </p> 
            </> )} 
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

      <form method='get' onSubmit={search}>
      <div>
      <h1>Search for items!</h1>
      <input type='text' name='String' placeholder='Search'/>
      <button name="Submit" type="submit">Search</button> 
      </div>     
      </form>
      <br/>
      <button className='knapp' onClick={sortName}>
        Sort Name
      </button>
      <button className='knapp' onClick={sortPrice}>
        Sort Price
      </button>
      <br></br>        
      <div className='card-container'>
        {!loading && products.map((value: Item) => (
          <Link to={`/item/${value.id}`} key={(value.id)}>
            <div className='card'>
              <img src={bird1} className="bird-logo" alt="bird1" />
              <div className='title'>
                {value.title}
              </div>
              <br/>
              <div className='price'>
                    {value.price + " " + CURRENCY}
                  </div>
       
              <img src={moreInfo} className='buynow-pic' alt="buynow"
                onClick={() => {

                  fetch(API_URL + "/cart", {
                    method: "PUT",
                    body: JSON.stringify({ item_id: value.id, amount: 1 }),
                    headers: new Headers({ "content-type": "application/json" })
                  })
                  const notify = (message: string) => {
                    toast.success(message);
                  };

                  notify('You just bought 1 ' + value.title);

                }} />
              <ToastContainer
                theme="dark"
                position="top-center"
                autoClose={3000} />
              <br></br>
            </div>
          </Link>
        ))}

      </div>
    </div>
  )
}

export default App
