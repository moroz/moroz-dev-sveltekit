import { paginatePosts } from "@api/blog";
import type { EntryGenerator, PageServerLoad } from "./$types";

export const load = (async ({ params: { page } }) => {
	const [posts, totalPages] = await paginatePosts(Number(page));

	return {
		title: "Blog",
		currentPage: Number(page),
		posts,
		totalPages,
	};
}) satisfies PageServerLoad;

export const prerender = true;

export const entries = (async () => {
	const [, totalPages] = await paginatePosts();
	return Array.from(new Array(totalPages - 1)).map((_, i) => ({
		page: String(i + 2),
	}));
}) satisfies EntryGenerator;
