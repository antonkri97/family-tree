import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { routeTree } from "./routeTree.gen";
import { useAuth, AuthProvider } from "./auth";
import { Skeleton } from "./components/ui/skeleton";

// Create a client
const queryClient = new QueryClient();

const router = createRouter({
  routeTree,
  defaultPreload: "intent",
  scrollRestoration: true,
  context: {
    auth: undefined!, // This will be set after we wrap the app in an AuthProvider
  },
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export function InnerApp() {
  const auth = useAuth();

  if (auth.initialized) {
    return <RouterProvider router={router} context={{ auth }} />;
  }

  return <Skeleton className="w-[100px] h-[20px] rounded-full" />;
}

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <InnerApp />
      </AuthProvider>
    </QueryClientProvider>
  );
}

// Render the app
const rootElement = document.getElementById("root")!;
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
