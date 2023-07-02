import { paginatePosts } from "@api/blog";
import type { PageServerLoad } from "./$types";

export const load = (async () => {
	const [posts, totalPages] = await paginatePosts(1);

	return {
		title: "Blog",
		posts,
		currentPage: 1,
		totalPages,
	};
}) satisfies PageServerLoad;
