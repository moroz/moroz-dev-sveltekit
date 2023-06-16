import { getSortedPostData } from "@api/blog";
import type { EntryGenerator } from "./$types";

export const prerender = true;

export const entries = (async () => {
	const posts = await getSortedPostData();
	return posts.map(({ slug }) => ({ slug }));
}) satisfies EntryGenerator;
