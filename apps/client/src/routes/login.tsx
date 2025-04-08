import { LandingHeader } from "@/components/landing-header";
import { LoginForm } from "@/components/login-form";
import { createFileRoute, redirect } from "@tanstack/react-router";
import { z } from "zod";

export const Route = createFileRoute("/login")({
  validateSearch: z.object({
    redirect: z.string().optional().catch(""),
  }),
  beforeLoad: ({ context, search }) => {
    if (context.auth.isAuthenticated) {
      throw redirect({ to: search.redirect || "/dashboard" });
    }
  },
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <LandingHeader className="mb-5" />
      <LoginForm />
    </>
  );
}
