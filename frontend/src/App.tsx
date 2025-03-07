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
