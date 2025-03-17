use crate::{
    db::models::Order,
    schema::{cart_items, items, order_items, orders},
};
use diesel::{prelude::*, result::Error};

use super::ShippingInformation;

#[derive(Queryable, Debug, Clone)]
struct CartItemWithItemData {
    pub item_id: i32,
    pub amount: i32,
    pub price: i32,
    pub discounted_price: Option<i32>,
    pub in_stock: i32,
}

// Attempts to create an order and return the id of the created order
pub fn create_order(conn: &mut MysqlConnection, user_id: i32, shipping_info: ShippingInformation) -> Result<i32, String> {
    // Run all operations as a transaction to keep it race condition and mutex safe
    let mut error_msg: Option<String> = None;
    let transaction_result = conn.transaction::<i32, diesel::result::Error, _>(|conn| -> Result<i32, Error> {
        // Check if user has a current ongoing order
        let ongoing_order_query = orders::table
            .filter(orders::user_id.eq(user_id))
            .select(Order::as_select())
            .first::<Order>(conn);
        if ongoing_order_query.is_ok() {
            error_msg = Some("User already has an ongoing order waiting for payment".to_string());
            return Err(Error::RollbackTransaction);
        } else {
            // Return error on any error other than NotFound (as this indicates no order exists and we may continue)
            let error = ongoing_order_query.unwrap_err();
            if error != Error::NotFound {
                error_msg = Some(format!("Failed to get ongoing order: {}", error.to_string()));
                return Err(Error::RollbackTransaction);
            }
        }

        
        // Get user's cart items to calculate order total
        let cart_items_result = cart_items::table
            .filter(cart_items::user_id.eq(user_id))
            .inner_join(items::table)
            .select((
                cart_items::item_id,
                cart_items::amount,
                items::price,
                items::discounted_price,
                items::in_stock,
            ))
            .load::<CartItemWithItemData>(conn);
        if cart_items_result.is_err() {
            error_msg = Some(format!("Failed to get cart items: {}", cart_items_result.unwrap_err().to_string()));
            return Err(Error::RollbackTransaction);
        }
    
        let cart_items = cart_items_result.unwrap();
        let mut total = 0;
    
        for cart_item in &cart_items {
            let price = match cart_item.discounted_price {
                Some(price) => price,
                None => cart_item.price,
            };
    
            total += price * cart_item.amount;
        }
    
        // Create order
        let order_data = (
            orders::user_id.eq(user_id),
            orders::address.eq(shipping_info.address),
            orders::co.eq(shipping_info.co),
            orders::zipcode.eq(shipping_info.zipcode),
            orders::country.eq(shipping_info.country),
            orders::total.eq(total),
            orders::comment.eq(shipping_info.comment),
            orders::payment_completed.eq(false),
        );
        let insert_order_result = diesel::insert_into(orders::table)
            .values(order_data)
            .execute(conn);
        if insert_order_result.is_err() {
            error_msg = Some(format!("Failed to create order: {}", insert_order_result.unwrap_err().to_string()));
            return Err(Error::RollbackTransaction);
        }
    
        // Get id of the order we just created
        let orderid_result = orders::table
            .filter(orders::user_id.eq(user_id))
            .select(orders::id)
            .first::<i32>(conn);
        if orderid_result.is_err() {
            error_msg = Some(format!("Failed to get order ID: {}", orderid_result.unwrap_err().to_string()));
            return Err(Error::RollbackTransaction);
        }
        let order_id = orderid_result.unwrap();
        
        // Move all cart items into order items
        for cart_item in &cart_items {
            // Reserve items in stock (will fail if not enough stock)
            let reservation_result = reserve_item_stock(conn, cart_item.clone());
            if reservation_result.is_err() {
                error_msg = Some(reservation_result.unwrap_err());
                return Err(Error::RollbackTransaction);
            }
            
            // Add order item
            let price = match cart_item.discounted_price {
                Some(price) => price,
                None => cart_item.price,
            };
    
            let order_item_values = (
                order_items::order_id.eq(order_id),
                order_items::item_id.eq(cart_item.item_id),
                order_items::amount.eq(cart_item.amount),
                order_items::total.eq(price * cart_item.amount),
            );
            let orderitem_insert_result = diesel::insert_into(order_items::table)
                .values(order_item_values)
                .execute(conn);
            if orderitem_insert_result.is_err() {
                error_msg = Some(format!("Failed to create order item (for item_id={}): {}", cart_item.item_id.to_string(), orderitem_insert_result.unwrap_err().to_string()));
                return Err(Error::RollbackTransaction);
            }
    
            // Remove cart item
            let cartitem_delete_result = diesel::delete(cart_items::table)
                .filter(cart_items::user_id.eq(user_id))
                .filter(cart_items::item_id.eq(cart_item.item_id))
                .execute(conn);
            if cartitem_delete_result.is_err() {
                error_msg = Some(format!("Failed to remove cart item (for item_id={}): {}", cart_item.item_id.to_string(), cartitem_delete_result.unwrap_err().to_string()));
                return Err(Error::RollbackTransaction);
            }
        }

        // TODO: Add timeout for order

        Ok(order_id)
    });

    // Handle error conditions
    match transaction_result {
        Ok(order_id) => Ok(order_id),
        Err(db_error) => match error_msg {
            Some(msg) => Err(msg),
            None => Err(db_error.to_string()),
        }
    }
}

pub fn cancel_order(conn: &mut MysqlConnection, order_id: i32) -> Result<(), String> {
    // Run all operations as a transaction to keep it race condition and mutex safe
    let mut error_msg: Option<String> = None;
    let transaction_result = conn.transaction::<(), diesel::result::Error, _>(|conn| -> Result<(), Error> {
        // Get order items to release the reserved stock
        // Get user's cart items to calculate order total
        let order_items_result = order_items::table
            .filter(order_items::order_id.eq(order_id))
            .inner_join(items::table)
            .select((
                order_items::item_id,
                order_items::amount,
                items::price,
                items::discounted_price,
                items::in_stock,
            ))
            .load::<CartItemWithItemData>(conn);
        if order_items_result.is_err() {
            error_msg = Some(format!("Failed to get cart items: {}", order_items_result.unwrap_err().to_string()));
            return Err(Error::RollbackTransaction);
        }
        
        // Release the stock of all items
        let order_items = order_items_result.unwrap();
        for order_item in order_items {
            let release_result = release_item_stock(conn, order_item);
            if release_result.is_err() {
                error_msg = Some(release_result.unwrap_err());
                return Err(Error::RollbackTransaction);
            }
        }

        // Delete order items
        let delete_order_items_result = diesel::delete(order_items::table)
            .filter(order_items::order_id.eq(order_id))
            .execute(conn);
        if delete_order_items_result.is_err() {
            error_msg = Some(format!("Order Items deletion failed: {}", delete_order_items_result.unwrap_err().to_string()));
            return Err(Error::RollbackTransaction)
        }
    
        // Delete order
        let delete_order_result = diesel::delete(orders::table.find(order_id)).execute(conn);
        if delete_order_result.is_err() {
            error_msg = Some(format!("Order deletion failed: {}", delete_order_items_result.unwrap_err().to_string()));
            return Err(Error::RollbackTransaction)
        }

        Ok(())
    });

    // Handle error conditions
    match transaction_result {
        Ok(order_id) => Ok(order_id),
        Err(db_error) => match error_msg {
            Some(msg) => Err(msg),
            None => Err(db_error.to_string()),
        }
    }
}

// Attempts to reserve stock for the given item, will return Err if not enough stock
// WARNING: Must be run inside a transaction to prevent race conditions!
fn reserve_item_stock(conn: &mut MysqlConnection, cart_item: CartItemWithItemData) -> Result<(), String> {
    if cart_item.in_stock < cart_item.amount {
        return Err(format!("Not enough items in stock"));
    }

    // Update item stock
    match diesel::update(items::table)
        .filter(items::id.eq(cart_item.item_id))
        .set(items::in_stock.eq(items::in_stock - cart_item.amount))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// Attempts to release stock for the given item
// WARNING: Must be run inside a transaction to prevent race conditions!
fn release_item_stock(conn: &mut MysqlConnection, order_item: CartItemWithItemData) -> Result<(), String> {
    // Update item stock
    match diesel::update(items::table)
        .filter(items::id.eq(order_item.item_id))
        .set(items::in_stock.eq(items::in_stock + order_item.amount))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
