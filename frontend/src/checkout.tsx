import { useEffect, useState } from "react";
import { API_URL } from "./etc/api_url";
import { toast } from "react-toastify";






export default function checkout() {

    const [products, setProducts] = useState<Array<CartItem>>([]);
    const [loading, setLoading] = useState<boolean>(true);

    const loadProd = async () => {
    const itemRequest = await fetch(API_URL + "/cart", {method: "GET"});
    if (itemRequest.ok) {
      const items: Array<CartItem> = await itemRequest.json();
      setProducts(items);
    }
    setLoading(false);
  }

  
  // Load products once at start of session
  useEffect(() => {loadProd();}, [])

  function totalPrice() {
    let total = 0;
    products.forEach(product => total += product.discounted_price ? product.discounted_price * product.amount : product.price);
    return total;
  }

  async function handleSubmit(e: any) {
    // By default, the browser will send the form data to the current URL,
    // and refresh the page. You can override that behavior by calling below
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);
    

    // Pass formdata as fetch body 
    const formJson = Object.fromEntries(formData.entries());
    const result = await fetch(API_URL + "/account", { headers: { "Content-Type": "application/json" }, method: "put", body: JSON.stringify(formJson) });
    console.log(result);

    await fetch(API_URL + "/order/create", { headers: { "Content-Type": "application/json" }, method: "post", body: JSON.stringify(formJson) });
  }

return(
    <div>
    <h2 className="price">
        totalt price is: 
        <br></br>
        {!loading && totalPrice()}   ₺
    </h2>
   <form method="post" onSubmit={handleSubmit} >
    <div>
          <h1>Login</h1>
          <input type="text" name="address" placeholder="Address" required/>
          <br />
          <input type="text" name="zipcode" placeholder="zipcode" required/>
          <br />
          <input type="text" name="co" placeholder="co" required/>
          <br />
          <input type="text" name="country" placeholder="country" required/>
          <br />         
          <button name="Submit" type="submit">Purchase        
          </button>
          </div>
    </form>
    </div>
);

}
