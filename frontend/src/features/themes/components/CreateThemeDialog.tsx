// Themeの新規作成用のDialog Component

import { useState } from "react";
import { useCreateTheme } from "../hooks/useThemes";
import { Loader2, X } from "lucide-react";

// コンポーネントなのでまずはProps（関数でいう引数に相当するもの）を定義
type Props = {
  // このコンポーネントの表示を切り替えるフラグ
  isOpen: boolean;
  // コンポーネントを閉じるための処理
  onClose: () => void;
};

// 設定したPropsを渡してコンポーネントを定義
export function CreateThemeDialog({ isOpen, onClose }: Props) {
  // 新規作成用に入力する項目の状態管理
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  // 成功時の処理を引数に渡して、APIへリクエストする用の変数を定義
  const { mutate, isPending } = useCreateTheme(() => {
    // 状態のリセット
    setTitle("");
    setDescription("");
    // コンポーネントを閉じる
    onClose();
  });

  // 送信(登録)ボタンを押した時の処理
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!title) return;

    // APIを叩くフックを起動
    mutate({ title, description });
  };

  // isOpenがFalseの場合はコンポーネントは何も表示しない(nullを返す)
  if (!isOpen) return null;

  // コンポーネントの定義
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-in fade-in duration-50">
      <div className="w-full max-w-lg rounded-md bg-white p-6 shadow-md animate-in zoom-in-95 duration-200">
        {/* コンポーネントのヘッダ */}
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl font-bold text-slate-900">Create New Theme</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* title */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Title <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-1.5 text-sm
              focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="e.g. NPS200% up!!"
              required
            />
          </div>

          {/* description */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Description
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-1.5 text-sm
              focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="description ... (optional)"
            />
          </div>

          {/* ボタンたち */}
          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isPending}
              className="flex items-center rounded-md bg-slate-900 px-4 py-2 font-sm text-white  font-medium hover:bg-slate-800 disabled:opacity-50"
            >
              {isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Create
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
