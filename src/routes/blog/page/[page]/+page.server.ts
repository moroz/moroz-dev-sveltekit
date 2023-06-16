import { paginatePosts } from "@api/blog";
import type { PageServerLoad } from "./$types";

export const load = (async ({ params: { page } }) => {
	const [posts, totalPages] = await paginatePosts(Number(page));

	return {
		posts,
		totalPages,
	};
}) satisfies PageServerLoad;
