import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { api } from "@/lib/api";
import { useMutation } from "@tanstack/react-query";
import type { AxiosError } from "axios";
import { useState, type ChangeEvent } from "react";
import { Link, useNavigate } from "react-router-dom";

export function Signup() {
  const navigate = useNavigate();

  // フォームの入力値管理
  const [formData, setFormData] = useState({
    employee_id: "",
    username: "",
    name: "",
    email: "",
    password: "",
  });

  // signupのAPIを叩く関数の定義
  const mutation = useMutation({
    mutationFn: async (newUser: typeof formData) => {
      return await api.post("/signup", newUser);
    },
    onSuccess: () => {
      alert("Account created! Please login.");
      navigate("/login");
    },
    onError: (error: AxiosError<{ error: string }>) => {
      console.log(error);
      alert(
        "Failed to create account. " +
          (error.response?.data?.error || "Unknown error")
      );
    },
  });

  // 入力欄が変わった時の処理
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setFormData({ ...formData, [e.target.id]: e.target.value });
  };

  // 送信ボタンを押した時の処理
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    mutation.mutate(formData);
  };

  return (
    <div className="flex h-screen items-center justify-center bg-slate-50">
      <Card className="w-140">
        <CardHeader>
          <CardTitle>Create an account</CardTitle>
          <CardDescription>
            Enter your information to create your account
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="space-y-2">
              <Label htmlFor="employee_id">Employee ID</Label>
              <Input
                id="employee_id"
                placeholder="5桁の数字を入力"
                required
                value={formData.employee_id}
                onChange={handleChange}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="username">Account Name</Label>
              <Input
                id="username"
                required
                value={formData.username}
                onChange={handleChange}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="name">Name</Label>
              <Input
                id="name"
                required
                value={formData.name}
                onChange={handleChange}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="email">Email</Label>
              <Input
                id="email"
                type="email"
                placeholder="****@example.com"
                required
                value={formData.email}
                onChange={handleChange}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="password">Password</Label>
              <Input
                id="password"
                type="password"
                required
                value={formData.password}
                onChange={handleChange}
              />
            </div>

            <Button
              type="submit"
              className="w-full"
              disabled={mutation.isPending}
            >
              {mutation.isPending ? "Creating..." : "Create account"}
            </Button>
          </form>
        </CardContent>
        <CardFooter className="flex justify-center">
          <p className="text-sm text-slate-500">
            Already have an account?{" "}
            <Link to="/login" className="text-blue-600 hover:underline">
              Login
            </Link>
          </p>
        </CardFooter>
      </Card>
    </div>
  );
}
