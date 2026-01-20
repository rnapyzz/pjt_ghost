import { api } from "@/lib/api";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { CreateSegmentPayload, Segment } from "../types";

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

// 新規作成
export function useCreateSegment(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (input: CreateSegmentPayload) => {
      const { data } = await api.post<Segment>("/segments", input);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["segments"] });
      if (onSuccess) onSuccess();
    },
  });
}
