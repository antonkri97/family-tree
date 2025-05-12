import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { LoginFormSchema, UserModel, UserRaw } from "./models/auth";
import { api } from "./lib/api";
import { SuccessResponse } from "./models/shared";
import { AxiosResponse } from "axios";

export interface AuthContext {
  isAuthenticated: boolean;
  initialized: boolean;
  login: (
    values: LoginFormSchema,
  ) => Promise<AxiosResponse<{ user: UserRaw; status: "success" }>>;
  logout: () => void;
  user: UserModel | null;
  isPending: boolean;
}

const AuthContext = createContext<AuthContext | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<UserModel | null>(null);
  const queryClient = useQueryClient();

  const { data, isFetching, isFetched } = useQuery({
    queryKey: ["me"],
    queryFn: async () => {
      try {
        const res = await api.get<{ status: string; user: UserRaw }>(
          "users/me",
        );
        return new UserModel(res.data.user);
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
      } catch (error) {
        UserModel.removeFromStorage();
        return null;
      }
    },
    retry: 1,
    staleTime: Infinity,
  });

  useEffect(() => setUser(data || null), [data]);

  const loginMutation = useMutation({
    mutationFn: (values: LoginFormSchema) =>
      api.post<{ user: UserRaw; status: "success" }>("auth/login", values),
    onSuccess: (res) => {
      const user = new UserModel(res.data.user);
      user.saveToStorage();
      setUser(user);
    },
  });

  const logoutMutation = useMutation({
    mutationFn: () => api.get<SuccessResponse<undefined>>("auth/logout"),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: ["me"] });
      UserModel.removeFromStorage();
      setUser(null);
    },
  });

  const login = useCallback(
    (values: LoginFormSchema) => loginMutation.mutateAsync(values),
    [loginMutation],
  );

  const logout = useCallback(() => logoutMutation.mutate(), [logoutMutation]);

  const isPending =
    loginMutation.isPending || logoutMutation.isPending || isFetching;

  return (
    <AuthContext.Provider
      value={{
        isAuthenticated: !!user,
        initialized: isFetched,
        user,
        login,
        logout,
        isPending,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
