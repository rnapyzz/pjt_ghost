import { api } from "@/lib/api";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { CreateThemeInput, Theme } from "../types";

// 一覧取得フック
export function useThemes() {
  return useQuery({
    queryKey: ["themes"],
    queryFn: async () => {
      const { data } = await api.get<Theme[]>("/themes");
      return data;
    },
  });
}

// 新規作成フック
export function useCreateTheme(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (input: CreateThemeInput) => {
      const { data } = await api.post<Theme>("/themes", input);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["themes"] });
      if (onSuccess) onSuccess();
    },
  });
}
