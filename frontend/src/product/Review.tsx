import { useState } from "react";
import { AuthUser } from "../auth/ProtectedRoute";
import { API_URL } from "../etc/api_url";

interface ReviewProps {
  review: ItemReviewWithComments,
  user: AuthUser | null,
  item_id: number,
  causeReviewsReload: () => void,
}

export default function Review({review, user, item_id, causeReviewsReload}: ReviewProps) {
  const [error, setError] = useState<string>("");
  const [showForm, setShowForm] = useState<boolean>(false);

  // Extract any direct children from review comments and pass the rest (not perfect but it works)
  const directChildren = review.comments.filter(comment => !comment.comment_id);
  const otherChildren = review.comments.filter(comment => comment.comment_id);

  async function deleteReview() {
    // Create a post request to API to delete the review
    const deleteResult = await fetch(API_URL + "/reviews?id=" + item_id, {
      method: "DELETE",
    })

    // If post succeeded, update array
    if (deleteResult.ok) {
      causeReviewsReload();
    }
  }

  async function handleSubmit(e: any) {
    e.preventDefault();
    setShowForm(false)

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);

    // Create a post request to API to create a review
    const newReview: NewComment = {
      comment: String(formData.get("comment")),
    }
    const createResult = await fetch(API_URL + "/comments?id=" + review.review_id, {
      method: "POST",
      body: JSON.stringify(newReview),
      headers: new Headers({"content-type": "application/json"})
    });

    // If post succeeded, add it to the reviews array
    if (!createResult.ok) {
      setError(await createResult.text());
    } else {
      setError("");
      causeReviewsReload();
    }
  }

  return (
    <div className="review-container">
      <div className="review">
        <h3 className="review-user">{review.firstname} {review.surname}:</h3>
        {[...Array(review.rating)].map(() => (<span className="review-text">*</span>))}
        {review.comment && <p className="review-text">{review.comment}</p>}

        {review.user_id === user?.user_id && (
          <button className="review-delete" onClick={() => deleteReview()}>Delete review</button>
        )}

        { user && (!showForm ? (
          <button className="review-respond-button" onClick={() => setShowForm(true)}>Respond</button>
        ) : (
          <button className="review-respond-button" onClick={() => setShowForm(false)}>Close</button>
        )) }
      </div>

      <div className="review-comments">
        {/* Write comment is only available to authed users */}
        { user && showForm && (
          <div className="review-comment">
            <h3 className="review-title">Respond to {review.firstname} {review.surname}</h3>
            {error && <p>{error}</p>}
            <form className="review-form" onSubmit={handleSubmit}>
              <input type="text" name="comment" placeholder="Response" />
              <br />
              <button>Submit</button>
            </form>
          </div>
        ) }

        { directChildren.map(comment =>
          <Comment
            comment = {comment}
            ancestorReviewId = {review.review_id}
            user = {user}
            possibleSubComments = {otherChildren}
            causeReviewsReload = {causeReviewsReload}
          />
        ) }
      </div>
    </div>
  );
}

interface CommentProps {
  comment: ReviewComment,
  ancestorReviewId: number,
  user: AuthUser | null,
  possibleSubComments: ReviewComment[],
  causeReviewsReload: () => void,
}

function Comment({ comment, ancestorReviewId, user, possibleSubComments, causeReviewsReload }: CommentProps) {
  const [error, setError] = useState<string>("");
  const [showForm, setShowForm] = useState<boolean>(false);

  async function deleteComment() {
    // Create a post request to API to delete the comment
    const deleteResult = await fetch(API_URL + "/comments?id=" + comment.id, {
      method: "DELETE",
    })

    // If post succeeded, update array
    if (deleteResult.ok) {
      causeReviewsReload();
    }
  }

  async function handleSubmit(e: any) {
    e.preventDefault();
    setShowForm(false)

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);

    // Create a post request to API to create a comment
    const newComment: NewComment = {
      comment: String(formData.get("comment")),
      parent_id: comment.id,
    }
    const createResult = await fetch(API_URL + "/comments?id=" + ancestorReviewId, {
      method: "POST",
      body: JSON.stringify(newComment),
      headers: new Headers({"content-type": "application/json"})
    });

    // If post succeeded, update array
    if (!createResult.ok) {
      setError(await createResult.text());
    } else {
      setError("");
      causeReviewsReload();
    }
  }

  // Extract any direct children from review comments and pass the rest (not perfect but it works)
  const directChildren = possibleSubComments.filter(subComment => subComment.comment_id === comment.id);
  const otherChildren = possibleSubComments.filter(subComment => subComment.comment_id !== comment.id);

  return (
    <div className="review-comment-container">
      <div className="review-comment">
        <h3 className="review-user">{comment.firstname} {comment.surname}:</h3>
        <p className="review-text">{comment.comment}</p>

        { comment.user_id === user?.user_id && (
          <button className="review-delete" onClick={() => deleteComment()}>Delete review</button>
        )}

        { user && (!showForm ? (
          <button className="review-respond-button" onClick={() => setShowForm(true)}>Respond</button>
        ) : (
          <button className="review-respond-button" onClick={() => setShowForm(false)}>Close</button>
        )) }
      </div>

      <div className="review-comments">
        {/* Write comment is only available to authed users */}
        { user && showForm && (
          <div className="review-comment">
            <h3 className="review-title">Respond to {comment.firstname} {comment.surname}</h3>
            {error && <p>{error}</p>}
            <form className="review-form" onSubmit={handleSubmit}>
              <input type="text" name="comment" placeholder="Response" />
              <br />
              <button>Submit</button>
            </form>
          </div>
        )}

        { directChildren.map(comment =>
          <Comment
            comment = {comment}
            ancestorReviewId = {ancestorReviewId}
            user = {user}
            possibleSubComments = {otherChildren}
            causeReviewsReload = {causeReviewsReload}
          />
        ) }
      </div>
    </div>
  );
}
