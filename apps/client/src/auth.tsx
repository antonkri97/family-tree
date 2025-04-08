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

export interface AuthContext {
  isAuthenticated: boolean;
  login: (values: LoginFormSchema) => void;
  logout: () => void;
  user: UserModel | null;
  isPending: boolean;
}

const AuthContext = createContext<AuthContext | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<UserModel | null>(null);
  // const queryClient = useQueryClient();

  const { data, isFetching } = useQuery({
    queryKey: ["me"],
    queryFn: async () => {
      try {
        console.log("Запрос 'me' начался");
        // const res = await Promise.resolve<{
        //   data: { status: string; user: UserRaw };
        // }>({
        //   data: {
        //     status: "success",
        //     user: {
        //       created_at: "2025-03-22T15:12:09.369682",
        //       email: "test-5@mail.com",
        //       id: "0018e094-f920-457d-ad14-7b366f551b05",
        //       name: "test-5",
        //       photo: "default.png",
        //       provider: "local",
        //       role: "user",
        //       updated_at: "2025-03-22T15:12:09.369682",
        //       verified: false,
        //     },
        //   },
        // });
        const res = await api.get<{ status: string; user: UserRaw }>(
          "users/me",
        );
        console.log("Запрос 'me' закончился");
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
      UserModel.removeFromStorage();
      setUser(null);
    },
  });

  const login = useCallback(
    (values: LoginFormSchema) => loginMutation.mutate(values),
    [loginMutation],
  );

  const logout = useCallback(() => logoutMutation.mutate(), [logoutMutation]);

  const isPending =
    loginMutation.isPending || logoutMutation.isPending || isFetching;

  return (
    <AuthContext.Provider
      value={{
        isAuthenticated: !!user,
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
