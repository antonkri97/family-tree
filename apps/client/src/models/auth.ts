import { z } from "zod";
import { Storage } from "./shared";

export const loginFormSchema = z.object({
  email: z.string().email({
    message: "Необходимо ввести email",
  }),
  password: z.string({
    message: "Необходимо ввести пароль",
  }),
});

export type LoginFormSchema = z.infer<typeof loginFormSchema>;

export const UserRawSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string(),
  verified: z.boolean(),
  photo: z.string(),
  provider: z.enum(["local", "Google"]),
  role: z.literal("user"),
  created_at: z.string(),
  updated_at: z.string(),
});

export type UserRaw = z.infer<typeof UserRawSchema>;

export class UserModel {
  id: string;
  name: string;
  email: string;
  verified: boolean;
  photo: string;
  provider: "local" | "Google";
  role: "user";
  createdAt: string;
  updatedAt: string;

  static storageKey: string = "user";

  constructor(private raw: UserRaw) {
    const checked = UserRawSchema.parse(raw);

    this.id = checked.id;
    this.name = checked.name;
    this.email = checked.email;
    this.verified = checked.verified;
    this.photo = checked.photo;
    this.provider = checked.provider;
    this.role = checked.role;
    this.createdAt = checked.created_at;
    this.updatedAt = checked.updated_at;
  }

  saveToStorage(): void {
    localStorage.setItem(UserModel.storageKey, JSON.stringify(this.raw));
  }

  static getFromStorage(): UserModel | null {
    const raw = localStorage.getItem(UserModel.storageKey);

    const check = UserRawSchema.safeParse(raw);

    if (check.success) {
      return new UserModel(UserRawSchema.parse(raw));
    }

    return null;
  }

  static removeFromStorage(): void {
    localStorage.removeItem(UserModel.storageKey);
  }
}
