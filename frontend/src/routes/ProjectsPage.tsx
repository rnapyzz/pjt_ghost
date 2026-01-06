import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useProjects } from "@/features/projects/api/getProjects";
import { CreateProjectDialog } from "@/features/projects/components/CreateProjectDialog";
import { Link } from "react-router";

export const ProjectPage = () => {
  const { data: projects, isLoading, isError, error } = useProjects();

  if (isLoading) {
    return <div className="text-center p-10">Loading projects...</div>;
  }

  if (isError) {
    return (
      <div className="text-red-500 p-10">
        Error: {error instanceof Error ? error.message : "Failed to fetch"}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-3xl font-bold tracking-tight">Projects</h2>
        <CreateProjectDialog />
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {projects?.map((project) => (
          <Link
            key={project.id}
            to={`/projects/${project.id}`}
            className="block h-full"
          >
            <Card
              key={project.id}
              className="hover:shadow-lg transition-shadow"
            >
              <CardHeader>
                <CardTitle className="flex justify-between items-start">
                  <span>{project.name}</span>
                </CardTitle>
                <CardDescription>
                  Created at {new Date(project.created_at).toLocaleDateString()}
                </CardDescription>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground whitespace-pre-wrap">
                  {project.description || "No description provided."}
                </p>
              </CardContent>
            </Card>
          </Link>
        ))}

        {projects?.length === 0 && (
          <p className="text-muted-foreground">
            No projects found. Create one!
          </p>
        )}
      </div>
    </div>
  );
};
