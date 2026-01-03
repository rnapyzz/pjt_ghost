import { api } from "@/lib/api";
import type { Project } from "@/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

// バックエンドに渡すデータ: リクエストのBody用の型定義
export type CreateProjectDTO = {
  name: string;
  description: string;
};

// useMutationで使用する非同期関数
const createProject = async (data: CreateProjectDTO): Promise<Project> => {
  // axiosを使うとオブジェクトを渡すだけでContent-Typeを自動設定してくれる
  const response = await api.post("/projects", data);
  return response.data;
};

// コンポーネントで使用するuseMutationを使ったproject作成用のカスタムフック
// ちなみに use動詞名詞 の命名はuseMutationであることを示唆する
export const useCreateProject = () => {
  // App.tsxで設定したクライアントを取得する（Context機能）
  const queryClient = useQueryClient();

  return useMutation({
    // useMutationが使用する非同期関数
    mutationFn: createProject,
    // 通信が成功した時の処理
    onSuccess: () => {
      // キャッシュを破棄して最新データを再取得する (ここで定義するとこのフック共通の処理にできる)
      queryClient.invalidateQueries({ queryKey: ["projects"] });
    },
  });
};
