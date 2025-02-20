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
}

export default ShoppingCart;