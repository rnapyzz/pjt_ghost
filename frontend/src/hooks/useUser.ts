import { api } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export type User = {
  id: string;
  employee_id: string;
  name: string;
  username: string;
  role: string;
};

export function useUser() {
  return useQuery({
    queryKey: ["me"],
    queryFn: async () => {
      const res = await api.get<User>("/me");
      return res.data;
    },
    enabled: !!localStorage.getItem("token"),
    retry: false,
  });
}
