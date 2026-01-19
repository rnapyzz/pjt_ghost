import { api } from "@/lib/api";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { CreateServicePayload, Service } from "../types";

// 一覧取得
export function useServices() {
  return useQuery({
    queryKey: ["services"],
    queryFn: async () => {
      const { data } = await api.get<Service[]>("/services");
      return data;
    },
  });
}

// 新規作成
export function useCreateService(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (input: CreateServicePayload) => {
      const { data } = await api.post<Service>("/services", input);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["services"] });
      if (onSuccess) onSuccess();
    },
  });
}
