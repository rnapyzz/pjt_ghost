// src/features/jobs/components/CreateJobDialog.tsx
import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useCreateJob } from "../api/createJob";
import { Textarea } from "@/components/ui/textarea";

// バリデーションスキーマ
const schema = z.object({
  name: z.string().min(1, "Job名は必須です"),
  description: z.string().optional(),
  business_model: z.enum([
    "saas",
    "ses",
    "contract",
    "media",
    "internal",
    "rnd",
  ]),
});

type FormValues = z.infer<typeof schema>;

type Props = {
  projectId: string;
};

export const CreateJobDialog = ({ projectId }: Props) => {
  const [open, setOpen] = useState(false);
  const { mutate, isPending } = useCreateJob(projectId);

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: "",
      description: "",
      business_model: "ses",
    },
  });

  const onSubmit = (data: FormValues) => {
    mutate(
      // undefinedの場合は空文字に変換
      { ...data, description: data.description || "" },
      {
        onSuccess: () => {
          setOpen(false);
          reset(); // フォームをクリア
        },
      }
    );
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>+ New Job</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-106.25">
        <DialogHeader>
          <DialogTitle>Create Job</DialogTitle>
          <DialogDescription>
            新しい案件を作成します。ビジネスモデルを選択してください。
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4 py-4">
          {/* Job Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Job Name</Label>
            <Input
              id="name"
              placeholder="例: A社開発案件 / SaaSプロダクト開発"
              {...register("name")}
            />
            {errors.name && (
              <p className="text-sm text-red-500">{errors.name.message}</p>
            )}
          </div>

          {/* Job Description */}
          <div className="space-y-2">
            <Label htmlFor="description">Description</Label>
            {/* Textareaがなければ <Input ... /> でもOK */}
            <Textarea
              id="description"
              placeholder="案件の詳細メモなど（任意）"
              {...register("description")}
            />
          </div>

          {/* Business Model (Native Select) */}
          <div className="space-y-2">
            <Label htmlFor="business_model">Business Model</Label>
            <select
              id="business_model"
              className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
              {...register("business_model")}
            >
              <option value="contract">Contract (受託開発)</option>
              <option value="ses">SES (準委任/人月型)</option>
              <option value="saas">SaaS (サブスクリプション型)</option>
              <option value="media">Media (メディア/広告枠販売)</option>
              <option value="internal">
                Internal (社内インフラ・社内活動)
              </option>
              <option value="rnd">RandD (研究開発)</option>
            </select>
            {errors.business_model && (
              <p className="text-sm text-red-500">
                {errors.business_model.message}
              </p>
            )}
          </div>

          <DialogFooter>
            <Button type="submit" disabled={isPending}>
              {isPending ? "Creating..." : "Create"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
