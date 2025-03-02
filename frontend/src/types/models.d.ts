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

interface PaginatedSearchQuery {
  page: number;
  per_page: number;
  search?: string;
}
