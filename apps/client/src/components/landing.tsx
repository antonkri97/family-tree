import { Button } from "@/components/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import {
  RocketIcon,
  FamilyIcon,
  Share2Icon,
  HistoryIcon,
} from "@/components/icons";
import { LandingHeader } from "./landing-header";

export function LandingPage() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-blue-50 to-white">
      {/* Navigation */}
      <LandingHeader />

      {/* Hero Section */}
      <section className="container flex flex-col items-center justify-center gap-8 px-4 py-16 text-center md:py-24">
        <div className="space-y-4">
          <h1 className="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl">
            Создайте свое <span className="text-primary">семейное древо</span>
          </h1>
          <p className="max-w-[600px] text-lg text-muted-foreground">
            Отслеживайте свою семейную историю, делитесь воспоминаниями и
            сохраняйте наследие для будущих поколений.
          </p>
        </div>
        <div className="flex gap-4">
          <Button size="lg" asChild>
            <a href="/register">Начать бесплатно</a>
          </Button>
          <Button size="lg" variant="outline" asChild>
            <a href="#features">Узнать больше</a>
          </Button>
        </div>
        <div className="mt-8 overflow-hidden rounded-xl border shadow-xl">
          <img
            src="/family-tree-preview.jpg"
            alt="Family Tree Preview"
            className="h-auto w-full max-w-4xl object-cover"
            width={1200}
            height={800}
          />
        </div>
      </section>

      {/* Features Section */}
      <section
        id="features"
        className="container space-y-12 px-4 py-16 md:py-24"
      >
        <div className="mx-auto max-w-3xl text-center">
          <h2 className="text-3xl font-bold sm:text-4xl">
            Возможности нашего сервиса
          </h2>
          <p className="mt-4 text-muted-foreground">
            Все, что вам нужно для создания и сохранения вашего семейного древа
          </p>
        </div>
        <div className="grid gap-8 md:grid-cols-2 lg:grid-cols-4">
          {FEATURES.map((feature) => (
            <Card
              key={feature.title}
              className="h-full transition-all hover:shadow-lg"
            >
              <CardHeader>
                <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
                  <feature.icon className="h-6 w-6 text-primary" />
                </div>
                <CardTitle className="mt-4">{feature.title}</CardTitle>
                <CardDescription>{feature.description}</CardDescription>
              </CardHeader>
            </Card>
          ))}
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary/5">
        <div className="container flex flex-col items-center justify-center gap-8 px-4 py-16 text-center md:py-24">
          <RocketIcon className="h-12 w-12 text-primary" />
          <h2 className="text-3xl font-bold sm:text-4xl">Готовы начать?</h2>
          <p className="max-w-[600px] text-lg text-muted-foreground">
            Присоединяйтесь к тысячам семей, которые уже сохранили свою историю
            с помощью нашего сервиса.
          </p>
          <Button size="lg" asChild>
            <a href="/register">Создать семейное древо</a>
          </Button>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t">
        <div className="container flex flex-col items-center justify-between gap-8 px-4 py-8 md:flex-row">
          <div className="flex items-center gap-2">
            <FamilyIcon className="h-6 w-6 text-primary" />
            <span className="text-lg font-semibold">FamilyTree</span>
          </div>
          <div className="text-sm text-muted-foreground">
            © {new Date().getFullYear()} FamilyTree. Все права защищены.
          </div>
        </div>
      </footer>
    </div>
  );
}

const FEATURES = [
  {
    title: "Простое создание",
    description: "Интуитивно понятный интерфейс для быстрого построения древа",
    icon: FamilyIcon,
  },
  {
    title: "Совместная работа",
    description: "Приглашайте родственников для совместного заполнения",
    icon: Share2Icon,
  },
  {
    title: "История семьи",
    description: "Добавляйте фото, документы и важные события",
    icon: HistoryIcon,
  },
  {
    title: "Разные форматы",
    description: "Экспорт в PDF, PNG или поделитесь ссылкой",
    icon: Share2Icon,
  },
];
