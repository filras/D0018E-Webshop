import payImg from "./assets/pay.png";
import cancelImg from "./assets/cancel.png";
import "./complete.css";
import { useEffect, useState } from "react";
import { API_URL } from "./etc/api_url";

export default function complete() {

    const [order, setOrderData] = useState<Order>();
    const [loading, setLoading] = useState<boolean>(true);
  
    const loadProd = async () => {
      const orderRequest = await fetch(API_URL + "/order/pending");
      if (orderRequest.ok) {
        const orders: Order = await orderRequest.json();
        setOrderData(orders);
      }
      setLoading(false);
    }
  
    
    // Load order once at start of session
    useEffect(() => {loadProd();}, [])
    
    
    return(
   
        <div className="comp">   
            <h1 className="comp-title">
                Your order:
            </h1>
            <br></br>
            
            <table className="comp-table">
                <tr>
                    <th>Order ID</th>
                    <th>Total price</th>
                    <th>Shipping Address</th> 
                </tr>
                    {
                    !loading && order && 
                        <tr key={order.id}>
                            <td>{order.id}</td> 
                            <td>{order.total}</td>
                            <td>{order.address}</td>
                        </tr>
                    }  
            </table>
            <img src={payImg} className="comp-pics"
                onClick={async () => {
                    let response = await fetch(API_URL + "/order/complete", {
                        method: "POST"
                    });
                    if (response instanceof Error) {
                        console.log("Something went wrong when buying order");
                    }
                    else if (response.ok) {
                        console.log("Order bought with GREAT success!");
                        window.location.href = "/";
                    }
                    
                }}/>
                <img src={cancelImg} className="comp-pics"
                    onClick={async () => {
                        let response = await fetch(API_URL + "/order/cancel", {
                            method: "DELETE"
                        });
                        if (response instanceof Error) {
                            console.log("Something went wrong when removing order");
                        }
                        else if (response.ok) {
                            console.log("Order removed with success");
                            window.location.href = "/";
                        }
                        
                    }}/>
          
        </div>
    )
}