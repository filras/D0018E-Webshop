import { useEffect, useState } from "react";
import shoppingcart from "./assets/shoppingcart.png";
import { API_URL } from "./etc/api_url";
import bird1 from "./assets/bird1.jpg";
//import removebutton from "./assets/deleteButton.gif";
import "./ShoppingCart.css";
import minus from "./assets/minusButton.png";
import plus from "./assets/plussbutton.png";



export default function ShoppingCart() {
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


  function totalPrice() {
    let total = 0;
    products.forEach(product => total += product.discounted_price ? product.discounted_price : product.price)
    return total;
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
      !loading && products.map((value: Item) =>(
        <div className='cart-product'>
              
              {<img src={bird1} className="cart-img" alt="bird-logo"/>
              }
                <div className="product-name"
                    key={(value.id)}>{value.title} 
                    <br></br>
                    id: {(value.id.toString())}
                    <br></br>
                  Description: {value.description}
                  <br></br>                
                  Discount: {value.discounted_price}
                  <br></br>
                  Rating: {value.average_rating}
                  <br></br>
                  Stock: {value.in_stock}
                </div>


            <img src={minus} className='cart-plus-minus' alt="cart-plus-minus"
              onClick={()=> {
                
              // Remove item from users shopping cart, update price
                 
                
              }}>

              </img>

            <img src={plus} className='cart-plus-minus' alt="cart-plus-minus"
              onClick={()=> {
                
              // Add item to users shopping cart, update price
                 
                
              }}>

            </img>


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
