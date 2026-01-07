// src/features/items/components/RowActionMenu.tsx

import { useState } from "react";
import { MoreHorizontal, Pencil, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { Item } from "@/types";
import { useUpdateItem, useDeleteItem } from "../api"; // index.tsからインポート

type Props = {
  projectId: string;
  jobId: string;
  item: Item;
};

export const RowActionMenu = ({ projectId, jobId, item }: Props) => {
  const [isEditOpen, setIsEditOpen] = useState(false);
  const [editName, setEditName] = useState(item.name);

  const updateMutation = useUpdateItem(projectId, jobId);
  const deleteMutation = useDeleteItem(projectId, jobId);

  const handleUpdate = () => {
    updateMutation.mutate(
      { itemId: item.id, data: { name: editName } },
      { onSuccess: () => setIsEditOpen(false) }
    );
  };

  const handleDelete = () => {
    // 誤操作防止のため、ブラウザ標準のconfirmを一応挟みます
    if (
      confirm(
        `「${item.name}」を削除しますか？\n入力済みの金額データも全て消えます。`
      )
    ) {
      deleteMutation.mutate(item.id);
    }
  };

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" className="h-8 w-8 p-0">
            <span className="sr-only">Open menu</span>
            <MoreHorizontal className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel>操作</DropdownMenuLabel>
          <DropdownMenuItem onClick={() => setIsEditOpen(true)}>
            <Pencil className="mr-2 h-4 w-4" />
            名前を変更
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={handleDelete}
            className="text-red-600 focus:text-red-600"
          >
            <Trash2 className="mr-2 h-4 w-4" />
            削除する
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {/* 名前変更ダイアログ */}
      <Dialog open={isEditOpen} onOpenChange={setIsEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>項目名を変更</DialogTitle>
          </DialogHeader>
          <div className="py-4">
            <Label htmlFor="name" className="text-right">
              項目名
            </Label>
            <Input
              id="name"
              value={editName}
              onChange={(e) => setEditName(e.target.value)}
              className="mt-2"
            />
          </div>
          <DialogFooter>
            <Button onClick={handleUpdate} disabled={updateMutation.isPending}>
              {updateMutation.isPending ? "保存中..." : "保存"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};
