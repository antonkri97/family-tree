import { LandingHeader } from "@/components/landing-header";
import { RegisterForm } from "@/components/register-form";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/register")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <LandingHeader className="mb-5" />
      <RegisterForm />
    </>
  );
}
