import { z, defineCollection } from "astro:content";

const blogCollection = defineCollection({
  schema: z.object({
    title: z.string(),
    date: z.date(),
    draft: z.boolean().optional(),
    summary: z.string().optional(),
  }),
});

const videoCollection = defineCollection({
  schema: z.object({
    title: z.string(),
    youtube: z.string(),
    date: z.date(),
    description: z.string().optional(),
  }),
});

export const collections = {
  blog: blogCollection,
  video: videoCollection,
};

export const POSTS_PER_PAGE = 20;
