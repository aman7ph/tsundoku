import { api } from "@/api/client";

export type Resource = {
  id: string;
  category_id: string;
  url: string;
  title: string | null;
  description: string | null;
  visited: boolean;
  shareable: boolean;
  created_at: string;
  updated_at: string;
};

export type ResourceChanges = { visited?: boolean; shareable?: boolean };

export async function listResources(categoryId: string): Promise<Resource[]> {
  const { data } = await api.get<Resource[]>(
    `/categories/${categoryId}/resources`,
  );
  return data;
}

export async function updateResource(
  id: string,
  changes: ResourceChanges,
): Promise<Resource> {
  const { data } = await api.patch<Resource>(`/resources/${id}`, changes);
  return data;
}
