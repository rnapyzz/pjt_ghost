import { useState } from "react";
import { useForm, useWatch } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery } from "@tanstack/react-query";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { getAccounts, getItemTypes, useCreateItem } from "../api";

const schema = z.object({
  accountId: z.string().min(1, "勘定科目を選択してください"), // UI制御用
  item_type_id: z.string().min(1, "項目種別を選択してください"),
  name: z.string().min(1, "項目名は必須です"),
  description: z.string().optional(),
});

type FormValues = z.infer<typeof schema>;

type Props = {
  projectId: string;
  jobId: string;
};

export const CreateItemDialog = ({ projectId, jobId }: Props) => {
  const [open, setOpen] = useState(false);
  const { mutate, isPending } = useCreateItem(projectId, jobId);

  // マスタデータの取得
  const { data: accounts } = useQuery({
    queryKey: ["accounts"],
    queryFn: getAccounts,
    staleTime: Infinity, // マスタは頻繁に変わらないのでキャッシュを長く持つ
  });

  const {
    register,
    handleSubmit,
    control,
    setValue,
    reset,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: "",
      description: "",
    },
  });

  // 選択された勘定科目IDを監視して、ItemTypeの選択肢を絞り込む
  const selectedAccountId = useWatch({
    control,
    name: "accountId",
  });

  // ItemTypeの取得（全件取得してJSでフィルタしても良いが、今回はAPIの絞り込み機能を使う例）
  const { data: itemTypes } = useQuery({
    queryKey: ["itemTypes", selectedAccountId],
    queryFn: () => getItemTypes(selectedAccountId),
    enabled: !!selectedAccountId, // Accountが選ばれるまで発火しない
  });

  const onSubmit = (data: FormValues) => {
    mutate(
      {
        item_type_id: data.item_type_id,
        name: data.name,
        description: data.description,
        assignee_id: null,
        entries: [], // 初期はデータなし
      },
      {
        onSuccess: () => {
          setOpen(false);
          reset();
        },
      }
    );
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button size="sm">+ Add Item</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-106.25">
        <DialogHeader>
          <DialogTitle>Add New Item</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4 py-4">
          {/* 1. Account Selection */}
          <div className="space-y-2">
            <Label>Account (勘定科目)</Label>
            <select
              className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm"
              {...register("accountId", {
                onChange: () => setValue("item_type_id", ""), // Accountが変わったらItemTypeをリセット
              })}
            >
              <option value="">Select Account...</option>
              {accounts?.map((acc) => (
                <option key={acc.id} value={acc.id}>
                  {acc.name} ({acc.category})
                </option>
              ))}
            </select>
            {errors.accountId && (
              <p className="text-sm text-red-500">{errors.accountId.message}</p>
            )}
          </div>

          {/* 2. ItemType Selection */}
          <div className="space-y-2">
            <Label>Item Type (項目種別)</Label>
            <select
              className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm disabled:opacity-50"
              {...register("item_type_id")}
              disabled={!selectedAccountId} // Account未選択時は無効化
            >
              <option value="">Select Item Type...</option>
              {itemTypes?.map((type) => (
                <option key={type.id} value={type.id}>
                  {type.name}
                </option>
              ))}
            </select>
            {errors.item_type_id && (
              <p className="text-sm text-red-500">
                {errors.item_type_id.message}
              </p>
            )}
          </div>

          {/* 3. Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Item Name (詳細名)</Label>
            <Input
              id="name"
              placeholder="例: ライセンス料 / サーバー費用"
              {...register("name")}
            />
            {errors.name && (
              <p className="text-sm text-red-500">{errors.name.message}</p>
            )}
          </div>

          <DialogFooter>
            <Button type="submit" disabled={isPending}>
              {isPending ? "Adding..." : "Add Item"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
