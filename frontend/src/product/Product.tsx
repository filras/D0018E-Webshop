import { useParams, useNavigate } from "react-router";

import bird1 from "../assets/bird1.jpg";
import buynow from "../assets/buynow.png";
import "./Product.css";
import { useEffect, useState } from "react";
import { API_URL } from "../etc/api_url";
import { CURRENCY } from "../etc/const";
import { AuthUser } from "../auth/ProtectedRoute";
import { toast } from "react-toastify";

interface Props {
  user: AuthUser | null;
}

export default function ProductPage({ user }: Props) {
  const itemId = Number(useParams().itemId);
  const navigate = useNavigate();

  const [product, setProduct] = useState<Item | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [reviews, setReviews] = useState<Array<ItemReview>>([]);
  const [loadingReviews, setLoadingReviews] = useState<boolean>(true);

  // Ignore invalid itemId by going to homepage
  if (!itemId) {
    navigate("/");
  }

  // Get product data from backend
  async function fetchProduct(id: number) {
    const result = await fetch(API_URL + "/item?id=" + id);

    if (result.ok) {
      const product = await result.json();
      setProduct(product);
    }
    setLoading(false);
  }

  // Get reviews from backend
  async function fetchReviews(id: number) {
    const result = await fetch(API_URL + "/reviews?id=" + id);

    if (result.ok) {
      const reviews = await result.json();
      setReviews(reviews);
    }
    setLoadingReviews(false);
  }

  const [error, setError] = useState<string>("");

  async function handleSubmit(e: any) {
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);

    // Create a post request to API to create a review
    const newReview: NewReview = {
      comment: String(formData.get("comment")),
      rating: Number(formData.get("rating")),
    }
    const createResult = await fetch(API_URL + "/reviews?id=" + itemId, {
      method: "POST",
      body: JSON.stringify(newReview),
      headers: new Headers({"content-type": "application/json"})
    })

    // If post succeeded, add it to the reviews array
    if (!createResult.ok) {
      setError(await createResult.text());
    } else {
      setError("");
      // This will only be run authed
      setReviews([...reviews, {
        ...newReview,
        firstname: user?.firstname || "",
        surname: user?.surname || "",
        user_id: user?.user_id || 0,
      }]);
    }
  }

  useEffect(() => {fetchProduct(itemId); fetchReviews(itemId)}, []);

  async function deleteComment() {
    // Create a post request to API to create an account
    const createResult = await fetch(API_URL + "/reviews?id=" + itemId, {
      method: "DELETE",
    })

    // If post succeeded, remove review from array
    if (createResult.ok) {
      // This will only be run authed
      setReviews(reviews.filter(review => review.user_id !== user?.user_id));
    }
  }

  return loading ? <div></div> : 
    <div className="product-container">
      <div className="product-box">
        <img src={bird1} alt="Product image" className="product-img" />
        <div className="product-title">
          <h1 className="product-title">{product?.title}</h1>
          <img src={buynow} className="product-add-to-cart" alt="Buy now"
            onClick={() => {
              fetch(API_URL + "/cart", {
                method: "PUT",
                body: JSON.stringify({ item_id: product?.id, amount: 1 }),
                headers: new Headers({ "content-type": "application/json" })
              })
              const notify = (message: string) => {
                toast.success(message);
              };

              notify(`Added ${product?.title} to cart`);
            }} />
        </div>
        <div className="product-desc">
          <p className="product-price">{product?.price} {CURRENCY}</p>
          <p className="product-price">Rating: {Math.round(product?.average_rating || 0)} / 5</p>
          <p className="product-text">{product?.description}</p>
        </div>
      </div>
      <div className="review-box">
        <h1>Reviews</h1>
        { loadingReviews ? <div></div> :
          reviews.map(review => (
            <div className="review">
              <h3 className="review-user">{review.firstname} {review.surname}:</h3>
              {[...Array(review.rating)].map(() => (<span className="review-text">*</span>))}
              {review.comment && <p className="review-text">{review.comment}</p>}

              {review.user_id === user?.user_id && (
                <button className="review-delete" onClick={() => deleteComment()}>Delete review</button>
              )}
            </div>
          ))
        }

        {/* Write review is only available to authed users */}
        { user && (
          <>
            <h2>Write review</h2>
            {error && <p>{error}</p>}
            <form className="review-form" onSubmit={handleSubmit}>
              Comment: <input type="text" name="comment" placeholder="Comment" />
              <br />
              Rating: <input type="number" min={1} max={5} name="rating" defaultValue={5} /> / 5
              <br />
              <button>Submit</button>
            </form>
          </>
        )}
      </div>
    </div>
}
