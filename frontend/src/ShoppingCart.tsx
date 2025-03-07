import { useEffect, useState } from "react";
import shoppingcart from "./assets/shoppingcart.png";
import { API_URL } from "./etc/api_url";
import bird1 from "./assets/bird1.jpg";
//import removebutton from "./assets/deleteButton.gif";
import "./ShoppingCart.css";
import minus from "./assets/minusButton.png";
import plus from "./assets/plussbutton.png";
import { toast } from "react-toastify";
import { ToastContainer } from "react-toastify";



export default function ShoppingCart() {

  const [products, setProducts] = useState<Array<CombinedCartItem>>([]);
  const [loading, setLoading] = useState<boolean>(true);

  const loadProd = async () => {
    const itemRequest = await fetch(API_URL + "/cart", {method: "GET"});
    if (itemRequest.ok) {
      const items: Array<CombinedCartItem> = await itemRequest.json();
      setProducts(items);
    }
    setLoading(false);
  }

  
  // Load products once at start of session
  useEffect(() => {loadProd();}, [])


  function totalPrice() {
    let total = 0;
    products.forEach(product => total += (product.discounted_price ? product.discounted_price  : product.price)* product.amount);
    return total;
  }


  async function increment(value: CartItem){
    value.amount += 1;
    await fetch(API_URL + "/cart", {
      method: "PUT",
      body: JSON.stringify(value),
      headers: {"Content-Type": "application/json"}
  });

  await loadProd();
  }

  async function decrement(value: CartItem){        

    value.amount -= 1;
    await fetch(API_URL + "/cart", {
      method: "PUT",
      body: JSON.stringify(value),
      headers: {"Content-Type": "application/json"}
  });
  
  await loadProd();
  }
 

  return (   
  <nav className="cart">
    <div className="cart-left">
      <h2 className="price">
        totalt price is: 
      <br></br>
        {totalPrice()}   ₺
      </h2>
      <a href="/checkout">
        <img src={shoppingcart} className='logo' alt='shoppingcart'/>
      </a>

    </div>

    <div className='cart-right'>
    <h1 className="cart-title">
      Your shopping cart
    </h1>
    <br></br>    
    { 
      !loading && products.map((value: CombinedCartItem) =>(  
        <div className='cart-product'>
              
              {<img src={bird1} className="cart-img" alt="bird-logo"/>
              }
                <div className="product-name"
                    key={(value.item_id)}>{value.title} 
                    <br></br>
                    id: {(value.item_id.toString())}
                    <br></br>
                  Description: {value.description}
                  <br></br>                
                  Discount: {value.discounted_price}
                  <br></br>
                  Rating: {value.average_rating}
                  <br></br>
                  Stock: {value.in_stock}
                  <br></br>
                  Amount: {value.amount}
                </div>


            <img src={minus} className='cart-plus-minus' alt="cart-plus-minus"
              onClick={()=> {
              decrement(value);
              const notify = (message: string) => {
                toast.success(message);
              };
              notify('You just added one ' + value.title);    
              }}>
            </img>
            <ToastContainer 
              theme="dark"
              position="top-center"
              autoClose={3000}/>  
            <img src={plus} className='cart-plus-minus' alt="cart-plus-minus"
              onClick={()=> {
              increment(value);
              const notify = (message: string) => {
                toast.success(message);
              };
              
              notify('You added one ' + value.title);
              }}>
            </img>
            <ToastContainer 
              theme="dark"
              position="top-center"
              autoClose={3000}/>  
            <div className='cart-price'>
              {value.price+"  ₺"}
              <br></br>        
            </div>                
            <br></br>
            </div>  
          ))}
    </div>
    
  </nav>
  );
};
