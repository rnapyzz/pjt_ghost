import { api } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";
import type { Segment } from "../types";

// 一覧取得
export function useSegments() {
  return useQuery({
    queryKey: ["segments"],
    queryFn: async () => {
      const { data } = await api.get<Segment[]>("/segments");
      return data;
    },
  });
}
