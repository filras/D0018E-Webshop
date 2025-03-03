import { PropsWithChildren } from "react";
import { Navigate } from "react-router";

export interface AuthUser {
  username: string;
  firstname: string;
  is_admin: boolean;
}

interface AuthUserProps {
  user: AuthUser | null;
  requireAdmin?: boolean;
  requireUnauthed?: boolean;
}

export function ProtectedRoute({ children, user, requireAdmin, requireUnauthed }: PropsWithChildren<AuthUserProps>) {
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
