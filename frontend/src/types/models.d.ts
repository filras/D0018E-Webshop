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
  user_id: number;
  item_id: number;
  comment?: string;
  rating: number;
}

interface NewReview {
  comment?: string;
  rating: number;
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

interface ItemReview {
  user_id: number;
  firstname: string;
  surname: string;
  comment?: string;
  rating: number;
}
