import { api } from "./client";

export type Category = {
  id: string;
  section_id: string;
  parent_id: string | null;
  name: string;
  icon: string | null;
  position: number;
  created_at: string;
};

export async function listCategories(
  sectionId: string,
  parentId?: string,
): Promise<Category[]> {
  const { data } = await api.get<Category[]>(
    `/sections/${sectionId}/categories`,
    {
      params: parentId ? { parent_id: parentId } : undefined,
    },
  );
  return data;
}

export async function createCategory(
  sectionId: string,
  name: string,
  parentId?: string,
): Promise<Category> {
  const { data } = await api.post<Category>(
    `/sections/${sectionId}/categories`,
    {
      name,
      parent_id: parentId ?? null,
    },
  );
  return data;
}
