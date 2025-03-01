import { Component } from "react";

// import API_URL from "./etc/api_url";


type MyProps = {
  
};

type MyState = {
  itemId: string;
  itemName: string;
};

class ShoppingCart extends Component<MyProps, MyState> {

  handleClick = (_event: MouseEvent): void => {
    alert('Button was clicked!');
};


  render(){
    return(
      <div>
         <h1>
          Shopping Cart
        </h1>
      </div>
      // Show total price - requirement = Kalix 
      //Show all items - requirement = Kalix
      // Let user remove items + update the price - requirement = Kalix
      // Let User go to "checkout" - requirement - Viggos mental health 

      );
  }
}

export default ShoppingCart;