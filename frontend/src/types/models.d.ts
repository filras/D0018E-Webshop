/* This file is generated and managed by tsync */

interface Item {
  id: number;
  title: string;
  description?: string;
  price: number;
  in_stock: number;
  average_rating?: number;
  discounted_price?: number;
}

interface NewItem {
  title: string;
  description?: string;
  price: number;
  in_stock: number;
  average_rating?: number;
  discounted_price?: number;
}

interface UpdateItem {
  title?: string;
  description?: string;
  price?: number;
  in_stock?: number;
  average_rating?: number;
  discounted_price?: number;
}

interface User {
  id: number;
  username: string;
  password_hash: string;
  firstname: string;
  surname: string;
  email: string;
  role: string;
  address?: string;
  zipcode?: string;
  co?: string;
  country?: string;
}

interface CartItems {
  user_id: number;
  item_id: number;
  amount: number;
}

interface NewUser {
  password: string;
  firstname: string;
  surname: string;
  email: string;
}

interface UpdateUser {
  firstname?: string;
  surname?: string;
  address?: string;
  zipcode?: string;
  co?: string;
  country?: string;
}

interface UpdateUserAsAdmin {
  username?: string;
  email?: string;
  firstname?: string;
  surname?: string;
  address?: string;
  zipcode?: string;
  co?: string;
  country?: string;
}

interface Review {
  id: number;
  user_id: number;
  item_id: number;
  comment?: string;
  rating: number;
}

interface Comment {
  id: number;
  user_id: number;
  review_id: number;
  comment_id?: number;
  comment: string;
}

interface OrderItem {
  order_id: number;
  item_id: number;
  amount: number;
  total: number;
}

interface Order {
  id: number;
  user_id: number;
  address: string;
  co?: string;
  zipcode: string;
  country: string;
  total: number;
  comment?: string;
  payment_completed: boolean;
}

interface PaginatedSearchQuery {
  page: number;
  per_page: number;
  search?: string;
}

interface PaginatedIdQuery {
  page: number;
  per_page: number;
  id: number;
}

interface CombinedCartItem {
  item_id: number;
  title: string;
  description?: string;
  price: number;
  in_stock: number;
  average_rating?: number;
  discounted_price?: number;
  amount: number;
}

interface UpdateCart {
  item_id: number;
  amount: number;
}

interface ShippingInformation {
  address: string;
  co?: string;
  zipcode: string;
  country: string;
  comment?: string;
}

interface OrderWithUserData {
  id: number;
  address: string;
  zipcode: string;
  co?: string;
  country: string;
  comment?: string;
  total: number;
  payment_completed: boolean;
  username: string;
  firstname: string;
  surname: string;
  email: string;
}

interface OrderWithUserDataAndItems {
  id: number;
  address: string;
  zipcode: string;
  co?: string;
  country: string;
  comment?: string;
  total: number;
  payment_completed: boolean;
  username: string;
  firstname: string;
  surname: string;
  email: string;
  items: Array<OrderItemWithData>;
}

interface OrderItemWithData {
  name: string;
  price: number;
  discounted_price?: number;
  amount: number;
  total: number;
}

interface ItemReviewWithComments {
  user_id: number;
  review_id: number;
  firstname: string;
  surname: string;
  comment?: string;
  rating: number;
  comments: Array<ReviewComment>;
}

interface ReviewComment {
  id: number;
  user_id: number;
  review_id: number;
  firstname: string;
  surname: string;
  comment: string;
  comment_id?: number;
}

interface NewReview {
  comment?: string;
  rating: number;
}

interface NewComment {
  comment: string;
  parent_id?: number;
}
