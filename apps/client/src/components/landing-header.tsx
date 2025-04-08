import { Link } from "@tanstack/react-router";
import { FamilyIcon } from "./icons";
import { Button } from "./ui/button";
import { cn } from "@/lib/utils";

export function LandingHeader({
  className,
  ...props
}: React.ComponentProps<"header">) {
  return (
    <header
      className={cn(
        "sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60",
        className,
      )}
      {...props}
    >
      <div className="container flex h-16 items-center justify-between px-4">
        <Link to="/">
          <div className="flex items-center gap-2">
            <FamilyIcon className="h-6 w-6 text-primary" />
            <span className="text-lg font-semibold">FamilyTree</span>
          </div>
        </Link>
        <nav className="flex items-center gap-4">
          <Button asChild variant="link">
            <Link to="/login">Войти</Link>
          </Button>
          <Button asChild variant="link">
            <Link to="/register">Зарегистрироваться </Link>
          </Button>
        </nav>
      </div>
    </header>
  );
}
