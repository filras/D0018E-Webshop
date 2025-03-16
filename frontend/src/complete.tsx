import payImg from "./assets/pay.png";
import cancelImg from "./assets/cancel.png";
import "./complete.css";
import { useEffect, useState } from "react";
import { API_URL } from "./etc/api_url";

export default function complete() {

    const [products, setProducts] = useState<Array<OrderItems>>([]);
    const [loading, setLoading] = useState<boolean>(true);
  
    const loadProd = async () => {
      const itemRequest = await fetch(API_URL + "/cart", {method: "GET"}); // Not implemented in backend yet
      if (itemRequest.ok) {
        const items: Array<OrderItems> = await itemRequest.json();
        setProducts(items);
      }
      setLoading(false);
    }
  
    
    // Load products once at start of session
    useEffect(() => {loadProd();}, [])
    
    /*
    async function cancel(value:OrderItems) {
        await fetch(API_URL + "/order/cancel",{
            method: "POST",
            body: JSON.stringify(value),
            headers: {"Content-Type": "application/json"}
        })
    }

    async function pay(value: OrderItems){
        await fetch(API_URL + "/order/complete",{
            method: "POST",
            body: JSON.stringify(value),
            headers: {"Content-Type": "application/json"}
        })
    }
    */
    
    return(
   
        <div className="comp">
            
            <h1 className="comp-title">
                Your order:
            </h1>
                  
           <img src={payImg} className="comp-pics"
                    onClick={async () => {
                        let response = await fetch(API_URL + "/order/complete",{
                            method: "POST",
                            body: JSON.stringify(value),
                            headers: {"Content-Type": "application/json"}
                        })
                        if(response instanceof Error){
                            console.log("Something went wrong when buying order");
                        }
                        else if(response.ok){
                            console.log("Order bought with GREAT success!");
                            window.location.href = "/";
                        }   
                        //pay(value);
                    }}/>

                    
            <img src={cancelImg} className="comp-pics"
                onClick={async () => {
                    let response = await fetch(API_URL + "/order/cancel",{
                        method: "POST",
                        body: JSON.stringify(value),
                        headers: {"Content-Type": "application/json"}
                    })
                    if(response instanceof Error){
                        console.log("Something went wrong when removing order");
                    }
                    else if(response.ok){
                        console.log("Order removed with success");
                        window.location.href = "/";
                    }
                    //cancel(value)
                }}  
            />
        </div>
    )
}