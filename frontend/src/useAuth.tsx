import { createContext, useContext, useState, PropsWithChildren } from "react";
import { Navigate } from "react-router";

export interface CurrentUser {
  authed: boolean;
  username?: string;
}

const AuthContext = createContext<CurrentUser | null>(null);

export function useCurrentUser() {
  const currentUserContext = useContext(AuthContext);

  if (!currentUserContext) {
    throw new Error(
      "useCurrentUser has to be used within <CurrentUserContext.Provider>"
    );
  }

  return currentUserContext;
};

function useAuth() {
  const [authed, setAuthed] = useState(false);

  return {
    authed,
    login() {
      return new Promise<void>((res) => {
        setAuthed(true);
        res();
      });
    },
    logout() {
      return new Promise<void>((res) => {
        setAuthed(false);
        res();
      });
    }
  };
}

export function AuthProvider({ children }: PropsWithChildren) {
  const auth = useAuth();

  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}

export default function AuthConsumer() {
  return useContext(AuthContext);
}
