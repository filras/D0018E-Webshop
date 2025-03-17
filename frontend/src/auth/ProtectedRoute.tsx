import { PropsWithChildren } from "react";
import { Navigate } from "react-router";

export interface AuthUser {
  user_id: number;
  username: string;
  firstname: string;
  surname: string;
  is_admin: boolean;
}

interface AuthUserProps {
  user: AuthUser | null;
  userLoading: boolean;
  requireAdmin?: boolean;
  requireUnauthed?: boolean;
}

export function ProtectedRoute({ children, user, userLoading, requireAdmin, requireUnauthed }: PropsWithChildren<AuthUserProps>) {
  if (userLoading) {
    return <h1>Loading...</h1>
  }
  
  // Enforce noauth on login/register page
  if (requireUnauthed && user !== null) {
    return <Navigate to="/" replace />
  }

  // Enforce admin or general auth
  else if (!requireUnauthed && (!user || (requireAdmin && !user.is_admin))) {
    return <Navigate to="/login" />
  }

  return <>{children}</>
}
