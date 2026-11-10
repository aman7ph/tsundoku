import { api } from "@/api/client";

export type Section = {
  id: string;
  name: string;
  icon: string | null;
  position: number;
  created_at: string;
};

export async function listSections(): Promise<Section[]> {
  const { data } = await api.get<Section[]>("/sections");
  return data;
}

export async function createSection(name: string): Promise<Section> {
  const { data } = await api.post<Section>("/sections", { name });
  return data;
}

export async function renameSection(
  id: string,
  name: string,
): Promise<Section> {
  const { data } = await api.patch<Section>(`/sections/${id}`, { name });
  return data;
}

export async function deleteSection(id: string): Promise<void> {
  await api.delete(`/sections/${id}`);
}
