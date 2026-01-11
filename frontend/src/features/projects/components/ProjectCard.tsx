// src/features/projects/components/ProjectCard.tsx

import { useState } from "react";
import { Link } from "react-router-dom";
import { ChevronDown, ChevronUp, ExternalLink, Loader2 } from "lucide-react";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { Badge } from "@/components/ui/badge"; // Jobのステータス表示などで使うかも（npx shadcn add badge）

import type { Project } from "@/types";
import { useJobs } from "@/features/jobs/api/getJobs";

interface ProjectCardProps {
  project: Project;
}

export const ProjectCard = ({ project }: ProjectCardProps) => {
  const [isOpen, setIsOpen] = useState(false);

  // カードが開いている時だけJobsを取得する
  const { data: jobs, isLoading } = useJobs({
    projectId: project.id,
    enabled: isOpen,
  });

  return (
    <Collapsible open={isOpen} onOpenChange={setIsOpen} className="h-full">
      <Card
        className={`h-full flex flex-col transition-all duration-200 ${
          isOpen ? "ring-2 ring-primary" : "hover:shadow-md"
        }`}
      >
        <CardHeader>
          <div className="flex items-start justify-between">
            <div className="space-y-1">
              <CardTitle className="text-xl line-clamp-1">
                {project.name}
              </CardTitle>
              <CardDescription>
                Created: {new Date(project.created_at).toLocaleDateString()}
              </CardDescription>
            </div>

            {/* 詳細ページへ飛ぶためのボタン（展開とは別にする） */}
            <Button variant="ghost" size="icon" asChild>
              <Link to={`/projects/${project.id}`}>
                <ExternalLink className="h-4 w-4 text-muted-foreground" />
              </Link>
            </Button>
          </div>
        </CardHeader>

        <CardContent className="flex-1">
          <p className="text-sm text-muted-foreground line-clamp-2 min-h-[2.5em]">
            {project.description || "No description provided."}
          </p>
        </CardContent>

        {/* 展開されたときに表示されるエリア */}
        <CollapsibleContent className="border-t bg-muted/20">
          <div className="p-4 space-y-3">
            <h4 className="text-sm font-semibold flex items-center gap-2">
              Jobs
              <Badge variant="secondary" className="text-xs">
                {jobs?.length || 0}
              </Badge>
            </h4>

            {isLoading ? (
              <div className="flex justify-center py-4">
                <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
              </div>
            ) : jobs?.length === 0 ? (
              <p className="text-xs text-muted-foreground py-2 text-center">
                No jobs yet.
              </p>
            ) : (
              <div className="grid grid-cols-1 gap-2">
                {jobs?.map((job) => (
                  <Link
                    key={job.id}
                    to={`/projects/${project.id}/jobs/${job.id}`}
                    className="block"
                  >
                    <div className="text-sm p-2 bg-background border rounded-md hover:bg-accent hover:text-accent-foreground transition-colors flex justify-between items-center">
                      <span className="truncate font-medium">{job.name}</span>
                      {/* ここに金額などを出しても良い */}
                    </div>
                  </Link>
                ))}
              </div>
            )}

            {/* Job追加ボタンなどをここに置くのもアリ */}
            <Button variant="outline" size="sm" className="w-full text-xs h-8">
              + Add Job
            </Button>
          </div>
        </CollapsibleContent>

        <CardFooter className="pt-2 pb-4 border-t bg-card rounded-b-xl z-10">
          <CollapsibleTrigger asChild>
            <Button variant="ghost" size="sm" className="w-full gap-2">
              {isOpen ? (
                <>
                  <ChevronUp className="h-4 w-4" /> Close
                </>
              ) : (
                <>
                  <ChevronDown className="h-4 w-4" /> Show Jobs
                </>
              )}
            </Button>
          </CollapsibleTrigger>
        </CardFooter>
      </Card>
    </Collapsible>
  );
};
