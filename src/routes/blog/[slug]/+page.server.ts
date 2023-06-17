import { formatMarkdown } from "$lib/markdown";
import { getPostDataBySlug, getSortedPostData } from "@api/blog";
import type { EntryGenerator, PageServerLoad } from "./$types";

export const prerender = true;

export const load = (async ({ params: { slug } }) => {
	const post = await getPostDataBySlug(slug);

	return {
		title: post.title,
		post: {
			...post,
			content: await formatMarkdown(post.content),
		},
	};
}) satisfies PageServerLoad;

export const entries = (async () => {
	const posts = await getSortedPostData();
	return posts.map(({ slug }) => ({ slug }));
}) satisfies EntryGenerator;
